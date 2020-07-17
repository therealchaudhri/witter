use crate::tests::test_helpers::*;

#[async_std::test]
async fn following_another_user() {
    let mut server = test_setup().await;

    let bobs_token = create_user_and_authenticate(&mut server, Some("bob".to_string()))
        .await
        .token;
    create_user_and_authenticate(&mut server, Some("alice".to_string())).await;

    for username in &["bob", "alice"] {
        let (json, status, _) = get(&format!("/users/{}/following", username))
            .send(&mut server)
            .await;
        assert_eq!(status, 200);
        assert_json_eq!(json, json!({ "data": [] }));

        let (json, status, _) = get(&format!("/users/{}/followers", username))
            .send(&mut server)
            .await;
        assert_eq!(status, 200);
        assert_json_eq!(json, json!({ "data": [] }));
    }

    let (json, status, _) = empty_post("/users/alice/follow")
        .header("Authorization", format!("Bearer {}", bobs_token))
        .send(&mut server)
        .await;
    assert_eq!(status, 201);
    assert_json_include!(actual: json, expected: json!({ "data": null }));

    let (json, status, _) = get("/users/bob/following").send(&mut server).await;
    assert_eq!(status, 200);
    assert_json_include!(
        actual: json,
        expected: json!({
            "data": [
                {
                    "username": "alice"
                }
            ]
        })
    );

    let (json, status, _) = get("/users/alice/following").send(&mut server).await;
    assert_eq!(status, 200);
    assert_json_eq!(
        json,
        json!({
            "data": []
        })
    );

    let (json, status, _) = get("/users/bob/followers").send(&mut server).await;
    assert_eq!(status, 200);
    assert_json_eq!(
        json,
        json!({
            "data": []
        })
    );

    let (json, status, _) = get("/users/alice/followers").send(&mut server).await;
    assert_eq!(status, 200);
    assert_json_include!(
        actual: json,
        expected: json!({
            "data": [
                {
                    "username": "bob"
                }
            ]
        })
    );
}

#[async_std::test]
async fn follow_same_user_twice() {
    let mut server = test_setup().await;

    let bobs_token = create_user_and_authenticate(&mut server, Some("bob".to_string()))
        .await
        .token;
    create_user_and_authenticate(&mut server, Some("alice".to_string())).await;

    let (_, status, _) = empty_post("/users/alice/follow")
        .header("Authorization", format!("Bearer {}", bobs_token))
        .send(&mut server)
        .await;
    assert_eq!(status, 201);

    let (json, status, _) = empty_post("/users/alice/follow")
        .header("Authorization", format!("Bearer {}", bobs_token))
        .send(&mut server)
        .await;
    assert_eq!(status, 422);
    assert_json_include!(
        actual: json,
        expected: json!({
            "error": {
                "message": "You cannot follow the same user twice",
            }
        })
    );
}

#[async_std::test]
async fn cannot_follow_self() {
    let mut server = test_setup().await;

    let bobs_token = create_user_and_authenticate(&mut server, Some("bob".to_string()))
        .await
        .token;

    let (json, status, _) = empty_post("/users/bob/follow")
        .header("Authorization", format!("Bearer {}", bobs_token))
        .send(&mut server)
        .await;
    assert_eq!(status, 422);
    assert_json_include!(
        actual: json,
        expected: json!({
            "error": {
                "message": "You cannot follow yourself",
            }
        })
    );
}
