(display "Hello Scheme\n")

(define (disk r)
  (lambda (p)
    (- (length p)
       r)))

;; pub fn torus(p: Vec2, r: Vec2) -> f32 {
    ;; (p.length() - r.x).abs() - r.y
; }
;;

(define (torus r)
  (lambda (p)
    (- (abs (- length
               (point-x p)))
       (point-y p))))

(define sdf (torus 0.5))

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
