FROM node:18.19.0

WORKDIR /app

# Copy package files
COPY server/package*.json ./

# Install dependencies
RUN npm install

# Create verilog directory and copy cpu.json
RUN mkdir -p /verilog
COPY verilog/cpu.json /verilog/

# Copy the rest of the application
COPY server/ .

# Expose the port the app runs on
EXPOSE 3000

# Start the application
CMD ["node", "index.js"] 