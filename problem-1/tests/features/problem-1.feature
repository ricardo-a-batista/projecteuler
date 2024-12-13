Feature: Euler Project - Problem 1: Multiples of 3 or 5

    Scenario: Find the sum of all the multiples of 3 or 5 below a natural number.
        Given all natural numbers below 10
        And a list of numbers
            | 3 |
            | 5 |
        Then the multiples of those numbers under the natural are
            | 3 |
            | 5 |
            | 6 |
            | 9 |
        And the sum of these numbers is 23
