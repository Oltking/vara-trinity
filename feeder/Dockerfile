FROM node:20-alpine
RUN npm install -g vara-wallet
WORKDIR /app
COPY package*.json ./
RUN npm ci --production
COPY dist ./dist
COPY idl ./idl
ENV NODE_ENV=production
CMD ["node", "dist/index.js"]
