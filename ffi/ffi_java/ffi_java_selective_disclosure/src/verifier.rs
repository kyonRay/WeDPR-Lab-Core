// Copyright 2020 WeDPR Lab Project Authors. Licensed under Apache-2.0.

//! Library of macros and functions for FFI of selective_disclosure solution,
//! targeting Java-compatible architectures (including Android).

use jni::{
    objects::{JClass, JObject, JString, JValue},
    sys::jobject,
    JNIEnv,
};
use protobuf::{self, Message};

use wedpr_ffi_common::utils::{
    java_jstring_to_bytes, java_new_jobject,
    java_set_error_field_and_extract_jobject,
};

use selective_disclosure;

use wedpr_protos::generated::selective_disclosure::{
    VerificationRequest, VerificationRule,
};

use wedpr_crypto::utils::bytes_to_string;

const RESULT_JAVA_CLASS_NAME: &str =
    "com/webank/wedpr/selectivedisclosure/VerifierResult";

fn get_result_jobject<'a>(_env: &'a JNIEnv) -> JObject<'a> {
    java_new_jobject(_env, RESULT_JAVA_CLASS_NAME)
}

#[no_mangle]
pub extern "system" fn Java_com_webank_wedpr_selectivedisclosure_NativeInterface_verifierGetRevealedAttrsFromVerificationRequest(
    _env: JNIEnv,
    _class: JClass,
    verification_request_jstring: JString,
) -> jobject
{
    let result_jobject = get_result_jobject(&_env);

    let verification_request_pb = java_safe_jstring_to_pb!(
        _env,
        result_jobject,
        verification_request_jstring,
        VerificationRequest
    );
    let revealed_attrs =
        match selective_disclosure::verifier::get_revealed_attrs_from_verification_request(
            &verification_request_pb,
        ) {
            Ok(v) => v,
            Err(_) => {
                return java_set_error_field_and_extract_jobject(
                    &_env,
                    &result_jobject,
                    "verifier get_revealed_attrs_from_verification_request \
                     failed",
                )
            },
        };

    java_safe_set_encoded_pb_field!(
        _env,
        result_jobject,
        revealed_attrs,
        "revealedAttributeInfo"
    );
    result_jobject.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_webank_wedpr_selectivedisclosure_NativeInterface_verifierVerifyProof(
    _env: JNIEnv,
    _class: JClass,
    verification_predicate_rule_jstring: JString,
    verification_request_jstring: JString,
) -> jobject
{
    // new jobject
    let result_jobject = get_result_jobject(&_env);

    let verification_predicate_rule_pb = java_safe_jstring_to_pb!(
        _env,
        result_jobject,
        verification_predicate_rule_jstring,
        VerificationRule
    );

    let verification_request_pb = java_safe_jstring_to_pb!(
        _env,
        result_jobject,
        verification_request_jstring,
        VerificationRequest
    );

    let result = match selective_disclosure::verifier::verify_proof(
        &verification_predicate_rule_pb,
        &verification_request_pb,
    ) {
        Ok(v) => v,
        Err(e) => {
            return java_set_error_field_and_extract_jobject(
                &_env,
                &result_jobject,
                &format!("verifier verify_proof failed, err = {:?}", e),
            )
        },
    };

    java_safe_set_boolean_field!(_env, result_jobject, result, "result");
    // if !result {
    //     return java_set_error_field_and_extract_jobject(
    //         &_env,
    //         &result_jobject,
    //         "verifier verify_proof failed, result is false",
    //     );
    // }
    result_jobject.into_inner()
}
