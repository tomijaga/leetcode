CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  RETURN (
        select min(salary)
        from 
            (select distinct salary from employee 
            group by salary
            order by salary desc
            limit N) t
        having count(*) >= N
        
  );
END