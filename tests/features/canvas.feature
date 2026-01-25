Feature: Canvas

  Scenario: Constructing the PPM header
    Given c ← canvas(5, 3)
    When ppm ← canvas_to_ppm(c)
    Then lines 1-3 of ppm are
      """
      P3
      5 3
      255
      """

  Scenario: Constructing the PPM pixel data
    Given c ← canvas(5, 3)
    And c1 ← color(1.5, 0, 0)
    And c2 ← color(0, 0.5, 0)
    And c3 ← color(-0.5, 0, 1)
    When write_pixel(c, 0, 0, c1)
    And write_pixel(c, 2, 1, c2)
    And write_pixel(c, 4, 2, c3)
    And ppm ← canvas_to_ppm(c)
    Then lines 4-6 of ppm are
      """
      255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
      0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
      0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
      """

  Scenario: PPM files are terminated by a newline character
    Given c ← canvas(5, 3)
    When ppm ← canvas_to_ppm(c)
    Then ppm ends with a newline character
