(display "Hello Scheme\n")

(define (disk r)
  (lambda (p)
    (~> (length p)
        (- r))))

(define (torus r)
  (lambda (p)
    (~> (length p)
        (- r)
        (abs)
        (- r))))

(define sdf (disk 0.3))

(define (run-sdf-iter grid i j)
  (cond
   ((= j NUM_Y) grid)
   ((= i NUM_X) (run-sdf-iter grid 0 (+ j 1)))
   (else
    (let ([p (grid->point i j)])
	    (grid-set grid i j (sdf p))
	    (run-sdf-iter
       grid
	     (+ 1 i)
	     j)))))

(define (run-sdf)
  (run-sdf-iter grid 0 0))
