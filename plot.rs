# Values for x and y axis
x <- 1:5
y <- x * x

# Using plot() function with additional settings
plot(x, y, type = "l", col = "blue", lwd = 2, xlab = "X-axis", 
	ylab = "Y-axis", main = "Quadratic Function")

# Add grid lines
grid()

# Add points to highlight data
points(x, y, col = "red", pch = 16)

# Add a legend
legend("topleft", legend = "y = x^2", col = "blue", lty = 1, lwd = 2, pch = 16)
