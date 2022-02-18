import "regenerator-runtime/runtime"
import React, { useEffect, useState } from "react"
import { login, logout } from "./utils"
import "./global.css"

import getConfig from "./config"
import { Button, Grid, Paper } from "@material-ui/core"

const imagesArr = [
	"https://i.pinimg.com/originals/21/c3/7f/21c37f5445a5296530e3ca91cdd5f42c.jpg",
	"https://i.kym-cdn.com/entries/icons/original/000/020/776/messi.jpg",
]

export default function App() {
	const [totalVotes, setTotalVotes] = useState([0, 0])
	const [hasUserVoted, setHasUserVoted] = useState([false, false])
	const [signedIn, setSignedIn] = useState(false)

	const countTotalVotes = async () => {
		const tV = await window.contract.get_total_votes()
		setTotalVotes(tV)
	}

	const getUserVote = async () => {
		const data = await window.contract.check_if_user_voted()
		setHasUserVoted(data)
	}

	const votePlayer = async (id) => {
		if (hasUserVoted === [false, false]) {
			switch (id) {
				case 0:
					await window.contract.vote_ronaldo()
				case 1:
					await window.contract.vote_messi()
			}
		}
	}

	useEffect(() => {
		countTotalVotes()
		getUserVote()
	}, [])

	useEffect(() => {
		setSignedIn(window.walletConnection.isSignedIn())
	}, [])

	return (
		<Grid
			container
			alignItems="center"
			direction="column"
			justifyContent="space-around"
			spacing={8}
		>
			<Grid item xs={12}>
				<Button
					style={{ margin: "auto" }}
					variant="contained"
					onClick={() => (signedIn ? logout() : login())}
				>
					{!signedIn ? "Sign In" : "Signout"}
				</Button>
			</Grid>
			<Grid style={{ fontSize: "2rem" }} item xs={12}>
				{window.walletConnection._authData.accountId}
			</Grid>
			<Grid
				container
				spacing={4}
				alignItems="center"
				justifyContent="space-around"
			>
				{imagesArr.map((img, ind) => (
					<Grid
						direction="column"
						container
						spacing={2}
						alignItems="center"
						item
						xs={5}
						key={ind}
					>
						<Grid item>
							<Paper style={{ padding: "1rem" }} variant="outlined">
								<img
									style={{ width: "100%", margin: "auto", height: "22rem" }}
									src={img}
								/>
							</Paper>
						</Grid>
						<Grid item>
							<Button onClick={() => votePlayer(ind)} variant="contained">
								{hasUserVoted[ind] === false ? "Vote" : "Voted"}
							</Button>
						</Grid>
						<Grid item style={{ margin: "1rem", fontSize: "1.5rem" }}>
							{totalVotes[ind]}
						</Grid>
					</Grid>
				))}
			</Grid>
		</Grid>
	)
}
