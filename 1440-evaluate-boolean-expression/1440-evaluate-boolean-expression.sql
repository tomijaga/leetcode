select 
    left_operand,
    operator,
    right_operand,
    if (operator like '=' and v1.value = v2.value, 
        "true",
       if (operator like '>' and v1.value > v2.value,
          "true",
          if ( operator like '<' and v1.value < v2.value,
             "true",
             "false"))) as value
from expressions e
inner join variables v1
on v1.name = e.left_operand
inner join variables v2
on right_operand = v2.name