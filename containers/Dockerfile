FROM docker.io/library/alpine:edge AS git
ARG TAG
ENV TAG=${TAG:-main}
RUN apk update && apk add --no-cache \
	git
WORKDIR /
RUN git -c advice.detachedHead=false clone --branch $TAG --depth=1 --recurse-submodules https://github.com/pfm-powerforme/joplin.git source
WORKDIR /source/


FROM docker.io/library/node:lts-alpine AS builder
RUN apk update && apk add --no-cache \
	gcc musl-dev g++ make py3-pip \
	tini \
	cairo-dev pango-dev libjpeg-turbo-dev giflib-dev librsvg-dev
RUN corepack enable
WORKDIR /build
COPY --from=git /source/.yarn/ ./.yarn/
COPY --from=git /source/package.json .
COPY --from=git /source/.yarnrc.yml .
COPY --from=git /source/yarn.lock .
COPY --from=git /source/gulpfile.js .
COPY --from=git /source/tsconfig.json .
COPY --from=git /source/packages/turndown ./packages/turndown
COPY --from=git /source/packages/turndown-plugin-gfm ./packages/turndown-plugin-gfm
COPY --from=git /source/packages/fork-htmlparser2 ./packages/fork-htmlparser2
COPY --from=git /source/packages/server/package*.json ./packages/server/
COPY --from=git /source/packages/fork-sax ./packages/fork-sax
COPY --from=git /source/packages/fork-uslug ./packages/fork-uslug
COPY --from=git /source/packages/htmlpack ./packages/htmlpack
COPY --from=git /source/packages/renderer ./packages/renderer
COPY --from=git /source/packages/tools ./packages/tools
COPY --from=git /source/packages/utils ./packages/utils
COPY --from=git /source/packages/lib ./packages/lib
COPY --from=git /source/packages/server ./packages/server
RUN sed --in-place '/onenote-converter/d' ./packages/lib/package.json
RUN mkdir -p /build/.yarn/cache \
    && mkdir -p /build/.yarn/berry/cache
RUN --mount=type=cache,target=/build/.yarn/cache \
    --mount=type=cache,target=/build/.yarn/berry/cache \
    BUILD_SEQUENCIAL=1 yarn config set cacheFolder /build/.yarn/cache
RUN yarn install --inline-builds


FROM docker.io/library/node:lts-alpine AS runtime
RUN apk update && apk add --no-cache \
    tini
RUN addgroup -g 977 joplin && \
    adduser -D -u 977 -G joplin -s /bin/ash joplin
USER joplin
COPY --chown=joplin:joplin --from=builder /build/packages /home/joplin/packages
ENV NODE_ENV=production
ENV RUNNING_IN_DOCKER=1
EXPOSE 3210
WORKDIR /home/joplin/packages/server
ENTRYPOINT ["tini", "--"]
CMD ["yarn", "start-prod"]
