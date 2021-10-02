# Agile Monte Carlo Simulations Demos

This is the repository which accompanies the blog post ["How to replace estimations and guesses with a Monte Carlo simulation"](https://lucasfcosta.com/2021/09/20/monte-carlo-forecasts.html).

It contains a few snippets which demonstrate how one could use Monte Carlo simulations to estimate the dates in which projects would end.

The different examples mentioned on the blog post are in different branches:

* `seven-sum-experiment` contains a Monte Carlo simulation to estimate the probability of obtaining a sum of 7 when rolling two dice.
* `naive-project-date-estimation` contains a Monte Carlo simulation which calculates the probability of one writing 60 or more blog posts in a year. It considers the blog posts' cycle times to be uniformly distributed.
* `non-naive-project-date-estimation` contains a Monte Carlo simulation which calculates the probability of one writing 60 or more blog posts in a year. Instead of considering the blog posts' cycle times to be uniformly distributed, it uses weighted probabilities.
* `throughput-project-date-estimation` uses throughput values to estimate when a project would be done. Such a simulation is similar to what you'd do to estimate whether a software development team could complete a project within a period.
* `histogram-plot` generates a histogram showing how frequently each completion date appeared when simulating the end date of a project multiple times.
* `percentiles-calculation` (same as `main`) simulates when a project would end, generates a histogram with the distribution of possible end dates, and calculates the values for the 25th, 50th, 75th, 85th, and 95th percentiles.

## Contributions welcome

I couldn't find a decent way to plot percentile lines into my histogram. I simply couldn't figure out how to do it using [Plotters](https://github.com/38/plotters).

In case you know how to do it, I'd appreciate it if you could open a PR showing me how.

If you're able to solve this problem, in addition to me being eternally grateful, you'll have your name added to the original blog post, and I'll update it with the plot containing percentile lines. Furthermore, I'll pay for coffee next time you're in London.

## Recommended Reading

* [GUIMARÃES CARVALHO, Gabriel. The art of solving problems with Monte Carlo simulations](https://ggcarvalho.dev/posts/montecarlo/)
* [VACANTI, Daniel. Actionable Agile Metrics For Predictability: An Introduction. ActionableAgile Press.](https://www.amazon.co.uk/dp/B013ZQ5TUQ)
* [VACANTI, Daniel. When Will It Be Done?: Lean-Agile Forecasting to Answer Your Customers’ Most Important Question. ActionableAgile Press.](https://www.amazon.co.uk/When-Will-Done-Lean-Agile-Forecasting-ebook/dp/B084WVMKLC)
