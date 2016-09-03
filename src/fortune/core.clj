(ns fortune.core
  (:gen-class))

(require '[clojure.string :as str])

(defn -main
  "print a fortune"
  [& args]
  (let [fortunes (slurp "fortunes")]
    (println (first (shuffle (str/split fortunes #"\n%\n"))))))
