DELIMITER //

CREATE PROCEDURE move_subtree(
    IN source_node_id INT,
    IN new_parent_id INT
)
BEGIN
    DECLARE source_left INT;
    DECLARE source_right INT;
    DECLARE source_width INT;
    DECLARE new_parent_right INT;

    -- Get the left, right, and width of the source subtree
    SELECT `lft`, `rgt`, `rgt` - `lft` + 1
    INTO source_left, source_right, source_width
    FROM nested_set_tree
    WHERE id = source_node_id;

    -- Get the right value of the new parent
    SELECT `rgt` INTO new_parent_right
    FROM nested_set_tree
    WHERE id = new_parent_id;

    -- Calculate the shift value based on the new parent's right value and the width of the subtree
    DECLARE shift_value INT;
    SET shift_value = new_parent_right - source_left + 1;

    -- Update the left and right values of the subtree to reflect the new position
    UPDATE nested_set_tree
    SET `lft` = `lft` + shift_value, `rgt` = `rgt` + shift_value
    WHERE `lft` BETWEEN source_left AND source_right;

    -- Adjust the left and right values of the moved subtree
    UPDATE nested_set_tree
    SET `lft` = `lft` - source_width
    WHERE `lft` > source_right;

    UPDATE nested_set_tree
    SET `rgt` = `rgt` - source_width
    WHERE `rgt` > source_right;
END //

DELIMITER ;