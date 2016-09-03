(defproject fortune "0.1.0-SNAPSHOT"
  :description "read and print a fortune"
  :dependencies [[org.clojure/clojure "1.8.0"]]
  :main ^:skip-aot fortune.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
