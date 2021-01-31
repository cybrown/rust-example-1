Feature: Posts

Scenario: Write a new post

    Given url 'http://localhost:3030/posts'
    And request { title: "foo", body: "bar" }
    When method post
    Then status 200
    And match response == { id: "#notnull", title: "foo", body: "bar", published: false }

    Given url 'http://localhost:3030/posts/' + response.id
    When method get
    Then status 200
    And match response == { id: "#notnull", title: "foo", body: "bar", published: false }
    And def postId = response.id

    Given url 'http://localhost:3030/posts/' + postId + '/published'
    And request ''
    When method put
    Then status 200
    And match response == { id: "#notnull", title: "foo", body: "bar", published: true }

    Given url 'http://localhost:3030/posts/' + response.id
    When method get
    Then status 200
    And match response == { id: "#notnull", title: "foo", body: "bar", published: true }

    Given url 'http://localhost:3030/posts/' + response.id + '/published'
    And request ''
    When method delete
    Then status 200
    And match response == { id: "#notnull", title: "foo", body: "bar", published: false }

    Given url 'http://localhost:3030/posts/' + response.id
    When method get
    Then status 200
    And match response == { id: "#notnull", title: "foo", body: "bar", published: false }
