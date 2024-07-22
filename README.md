# Strategist Terminal

![Strategist Terminal Logo](https://github.com/PeggyJV/strategist-terminal/blob/main/static/logo.png)

The Strategist Terminal is a graphical user interface for a stategist to execute rebalacing of strategies running on the Sommelier protocol.

The goals are 

- Graphical entry of parameters for a rebalance
- Tracking of the life cycle of a rebalance from submission to execution
- Management of the SSL certificate the authenticates a strategist

Non-goals
- Performance and monitoring of cellars.


## Status

The Strategy Terminal is **pre-alpha** development mode with the goal of a alpha release in August 2024.




## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
