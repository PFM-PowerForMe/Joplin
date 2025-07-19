FROM alpine:edge AS git
ARG TAG
ENV TAG=${TAG:-main}
RUN apk update && apk add --no-cache \
	git
WORKDIR /tmp/
RUN git -c advice.detachedHead=false clone  --branch $TAG --depth=1 --recurse-submodules https://github.com/pfm-powerforme/joplin.git joplin
WORKDIR /tmp/joplin/


FROM node:22-alpine AS builder
RUN apk update && apk add --no-cache \
	gcc musl-dev g++ make py3-pip \
	tini \
	cairo-dev pango-dev libjpeg-turbo-dev giflib-dev librsvg-dev
RUN corepack enable
WORKDIR /tmp/build
COPY --from=git /tmp/joplin/.yarn/ ./.yarn/
COPY --from=git /tmp/joplin/package.json .
COPY --from=git /tmp/joplin/.yarnrc.yml .
COPY --from=git /tmp/joplin/yarn.lock .
COPY --from=git /tmp/joplin/gulpfile.js .
COPY --from=git /tmp/joplin/tsconfig.json .
COPY --from=git /tmp/joplin/packages/turndown ./packages/turndown
COPY --from=git /tmp/joplin/packages/turndown-plugin-gfm ./packages/turndown-plugin-gfm
COPY --from=git /tmp/joplin/packages/fork-htmlparser2 ./packages/fork-htmlparser2
COPY --from=git /tmp/joplin/packages/server/package*.json ./packages/server/
COPY --from=git /tmp/joplin/packages/fork-sax ./packages/fork-sax
COPY --from=git /tmp/joplin/packages/fork-uslug ./packages/fork-uslug
COPY --from=git /tmp/joplin/packages/htmlpack ./packages/htmlpack
COPY --from=git /tmp/joplin/packages/renderer ./packages/renderer
COPY --from=git /tmp/joplin/packages/tools ./packages/tools
COPY --from=git /tmp/joplin/packages/utils ./packages/utils
COPY --from=git /tmp/joplin/packages/lib ./packages/lib
COPY --from=git /tmp/joplin/packages/server ./packages/server
# We don't want to build onenote-converter since it is not used by the server
RUN sed --in-place '/onenote-converter/d' ./packages/lib/package.json
RUN --mount=type=cache,target=/tmp/build/.yarn/cache --mount=type=cache,target=/tmp/build/.yarn/berry/cache\
    BUILD_SEQUENCIAL=1 yarn config set cacheFolder /tmp/build/.yarn/cache \
    && yarn install --inline-builds


FROM node:22-alpine AS runtime
RUN apk update && apk add --no-cache \
    tini
RUN addgroup -g 977 joplin && \
    adduser -D -u 977 -G joplin -s /bin/ash joplin
USER joplin
COPY --chown=joplin:joplin --from=builder /tmp/build/packages /home/joplin/packages
ENV NODE_ENV=production
ENV RUNNING_IN_DOCKER=1
EXPOSE 3210
WORKDIR /home/joplin/packages/server
ENTRYPOINT ["tini", "--"]
CMD ["yarn", "start-prod"]
