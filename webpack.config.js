const path = require('path');
const webpack = require('webpack');

module.exports =
{
	entry:
	{
		app: path.join(__dirname, "js/index.js")
	},
	output:
	{
		path: path.join(__dirname, "res/static/"), filename: "wp.js"
	},
	module:
	{
		rules: [
			{
				test:/\.(js|jsx)$/,
				loader: 'babel-loader',
				exclude: /node_modules/,
				query:
				{
					presets: ['@babel/preset-env', '@babel/preset-react']
				} 
			}
		]
	}
};