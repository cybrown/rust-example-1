Feature: Posts

Scenario: Write a new post, publish and unpublish it

    * def title = "title"
    * def body = "body"

    Given url root + '/posts'
    And request { title: #(title), body: #(body) }
    When method POST
    Then status 200
    And match response == { id: "#notnull", title: #(title), body: #(body), published: false }
    * def postId = response.id

    Given url root + '/posts/' + postId
    When method GET
    Then status 200
    And match response == { id: #(postId), title: #(title), body: #(body), published: false }

    Given url root + '/posts/' + postId + '/published'
    And request ''
    When method PUT
    Then status 200
    And match response contains { published: true }

    Given url root + '/posts/' + postId
    When method GET
    Then status 200
    And match response contains { published: true }

    Given url root + '/posts/' + postId + '/published'
    And request ''
    When method DELETE
    Then status 200
    And match response contains { published: false }

    Given url root + '/posts/' + postId
    When method GET
    Then status 200
    And match response contains { published: false }
