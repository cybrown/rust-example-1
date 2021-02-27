Feature: Posts

Scenario: Write a new post, publish and unpublish it

    * def postTitle = "title"
    * def postBody = "body"
    * def postSchema = { id: "#notnull", title: #string, body: #string, published: #boolean }

    Given url root + '/posts'
    And request { title: #(postTitle), body: #(postBody) }
    When method POST
    Then status 201
    And match response == postSchema
    And match response contains { title: #(postTitle), body: #(postBody), published: false }
    * def postId = response.id

    Given url root + '/posts/' + postId
    When method GET
    Then status 200
    And match response == postSchema
    And match response contains { id: #(postId), title: #(postTitle), body: #(postBody), published: false }

    Given url root + '/posts/' + postId + '/published'
    And request ''
    When method PUT
    Then status 200
    And match response == postSchema
    And match response contains { published: true }

    Given url root + '/posts/' + postId
    When method GET
    Then status 200
    And match response == postSchema
    And match response contains { published: true }

    Given url root + '/posts/' + postId + '/published'
    And request ''
    When method DELETE
    Then status 200
    And match response == postSchema
    And match response contains { published: false }

    Given url root + '/posts/' + postId
    When method GET
    Then status 200
    And match response == postSchema
    And match response contains { published: false }
