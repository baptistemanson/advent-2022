pub fn pb1() {
    let mut total: u32 = 0;
    for line in INPUT.lines() {
        assert!(line.len() % 2 == 0, "{} is not of even length", line);
        let (first_half, second_half) = &line.split_at(line.len() / 2);
        total += to_priority(find_missing(first_half, second_half)) as u32;
    }
    println!("{}", total);
}

fn to_priority(c: char) -> u8 {
    return if c.is_lowercase() {
        (c as u8) - ('a' as u8) + 1
    } else {
        (c as u8) - ('A' as u8) + 27
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        assert_eq!(to_priority('p'), 16);
        assert_eq!(to_priority('L'), 38);
        assert_eq!(to_priority('P'), 42);
        assert_eq!(to_priority('v'), 22);
        assert_eq!(to_priority('t'), 20);
    }
}
// could short circuit with a hashset to not scan twice the same.
fn find_missing(a: &str, b: &str) -> char {
    for to_find in a.chars() {
        if b.contains(to_find) {
            return to_find;
        }
    }
    panic!("couldnt find")
}

// could short circuit with a hashset to not scan twice the same.
fn find_missing_3(a: &str, b: &str, c: &str) -> char {
    for to_find in a.chars() {
        if b.contains(to_find) && c.contains(to_find) {
            return to_find;
        }
    }
    panic!("couldnt find")
}

pub fn pb2() {
    let mut total: u32 = 0;
    let mut i = 0;
    let mut l = [""; 3];
    for line in INPUT.lines() {
        l[i % 3] = line;
        if i % 3 == 2 {
            total += to_priority(find_missing_3(&l[0], &l[1], &l[2])) as u32;
        }
        i += 1;
    }
    println!("{}", total);
}

#[allow(dead_code)]
const INPUT: &str = "\
GbccTtTSGGbgrcWBGGrdgTnVQnCmNpCJlNnNPVfClcnN
vMzvZhzhwDLVmQnClwwNQp
FRsZFzjQFsqRzRRjDZbdtTgdHBBWGrdBdHHs
HCLTmbCLgzNBNPSSlT
JJGMWRJMrrdwWWGjGWMLRGLjBzNQsBzPPfflzDPBsBffDrQz
pwJdLMjdMddWjLtwZWMMwGtHhnvnCnhvqVFFZnvbgbqVCZ
tvMCDCSVVvDDBQFRbqWMMsWgFWgc
BLLPTpBmfLPrHLLfLsbhRqbzRRcRHgqssR
dfdNLmPTdNZmZdZPfpmTJLPPSvQjtSGVwQSDJSjSwDQBVCGw
wZWTWNFqzwZbWNpSgGMVMtTHsgGs
nlnPnPvLQjzdtsjBHBMMGSHg
LdnrrLnhRdLLmLDRPvmdQnJDJWNqcCqZJZqfFqfcfzcq
vPTbfWggzvGVqjsVqV
dDcJHZcZHmMFQQMshsjcRqVChjNtqh
dDHJDmFnrJmQFnBdMdQHJdZZlWTTPPjTLWbTzLWlTTfwjzBT
VfmDHDfZzfLcZLLLHBFQtRJTcdjgdTgFjjtR
WRPhMlGSshPRGgvFMtddTjjCQt
SPhWPsbNWShsWllswGpzLmzfZwmZfqLVRrDBZB
MtZgRgJJbbGjgDDgbjRjRbGcNdWwncBFdLBBMhFBQwnWnw
vlpsNVsCzzfHpvTpzlSSSvppcQdfnwnWhndWndwfQLhnhcFQ
pCsCCTHVvSzvPvHvzpPVTVHHRJJDgJZjJjqNmjZmDtJRZPNR
GTGTbhhPjJJjBhhZsGzmfHSNsmHHgSdL
FcFpMDFDRFfsRHSjmLjR
CpFjqcCjwjnpwhTPTWBQZZTb
zdzzwDlnTDQQQQnqQqqsFqnrSBSprbpjNJJBJPPdpJfbZb
hgMcRVGMtHgRcNSPrpfgfjJpBp
CLRHVHhtvtvGPWFFDqPDLnqP
jssjjjHCSGCMNJTWWCJT
DvcvBtVrrDSNWcMfLRfM
ppDZSpBhBvBmvDHnFsHHPnGPGbQp
VVVLsLWnjVVBGgScjtBjjLLgmJdPmJmfmhmGmmmNmJGhPNJP
QbqlZqQTZvMHmshvFhPfffFp
CsTRrQrwbCbrZqQTQlRjtDBSBtwBjgVWLBtgBt
FLsSFRTPscHZmGRGGc
npNNptgttCNpgLbnQMgnQnMNqVhGqZrmrmmqrmcrqmCVZwqH
WMWbtpjLgnLNvSfPPzjvjPdv
FcFFhZlhlMrHlSFSrHZMJZSVmmLmVLLCsBtLBCzCBVDRcV
PMGPbndvGfGstLzDCmLB
NwjPqdvPpvgddqgwHrZhJlTlThpWJMZp
SdjStScTWTwwvwwfjRhQPQQQDlLBGpLrPrLrLc
gCqJbNsVsNMgzMJnnqzNlCLGGlZZPrLLlLLtpPDl
JsFJFMmbJqqnJbhSHdjWwjtmHWvS
zBFDGGbNzDWRbDccsWslHlWWsJcS
zMVqTwzPfVfVMwmlcZTZZlSmTlmc
MrMvMCnrMVMCPrPnDFFGgQdFgRdzznbd
rJtJnrnSShJgcCsjjNNMSSDzRmzm
HWDWPBPDBfFVBffqplvlmNlQllvzQNQqlN
FVGHHFVbwBpBPwFFGfBpHVDgrcJCbZcLdCgtcCcJLJrd
MGHGGFFqbFTGmFwLmQsQflFN
WcvBdpjhdZdNwdZwLZ
vWtgVcpvjthtNcjntDhhpSJMSqHzqTzqCVHTSqHPGT
cVHZfjfZMcrSDQMJRCBCQw
PtGddtslsWQDBdwCDDdw
WsvTFnshPTGhGhhlPNGTCnsjgFVfmgfHZfVHgcHZVVFmFV
JFFqfJBgrHBffVHlsBFqfWNgjTtztNnttWWvWNwzwt
hGZbcBcZZSQmZLQRTbvNttTzjtTvbpzv
cmmhRchPZhZSSmdmGPDDdJdBVMffHlqlslfF
qWwTNwNHMHNNMRqMdRMQQMHLmmvzrTmrzPvzJvZvZlvzjZ
FphBpnBhVBSFvLljzZPpmrPL
nGsBbssbcbdlwggdNl
RLSRTLSFFFLPSWpzzTJdzsQpbd
DvqqcwVMDDcfrrnwDcwnvCdpQQphJhJjhdhpzsJhMQ
fGcvDZffcGGZDHGrGrtJRPlPmJSlPLRgNBHg
QlFFmGQFDQrrWlRlWGrnQVCLNvvPwLCwBvCcCcJCLCCm
tHtfsjSMCNPwzvCf
MjqStqMHsMSgjShjTttgphsTlrbDWGDrGlRTlNDbQrQRWRbD
QbChcCJCbHQCjbGCjQfsdsrtTqrfTLrcFftd
DwRzVzzZnzZRwvgRhRWtqsLLWtRdqLLdqd
NMwMzBVVPPSGQhBl
ttTPHWdrJjCdjnFMtLLtLNvQltLh
pSDBwZRBBsgfDGGsGpBVMFPQQlFMFQQFQfNvLNfF
PsppZzBVzwgDwBwwgpSSBssWjqrdCnjjHdmCTHznWCJJWn
WcdHdPcdZrLPDPBQDg
pMjMMqfmJlqNflMlFNRfLBwnLzTTTDJJwDTTGTLJ
hlhbqpbNNbVVdbZtSB
RDBWGRDnzBWBJDNBttSLlclldtQQcTTLFF
rTPVjZZsCZrVhdFMcgLgwFSgQh
PsjHVffbsTCHrCvTPfDJGJHNzzNJWnnnmzDB
LQdFgTLdQjVsQFTRBjMZrmBjWGMGSW
flvJJlJpbNnppCpMGGfBBZSZRfFmGr
NNbNDNlbDpHlbDDplvzvnCbzqsQPFQsTTccsqdQqLgdHLwQw
DnGDNDTFdFwDzCZZRmhThCRRRv
SgrPLrrLsBPbHBCmtVZVCdCcctHH
rSbgBrsqgsPppMBqfpPsLpPGlNGGDwNFNJWDldlllDwJMG
PWbvNWvpvJPnWDGqDjDczj
QwfFFVVQSMlDlQfFZhsHrBrhHHTcjnczqjzqrG
mwMSgfmDmSSFgfFNbmLpbRbJbvbRpC
lsggLLLDGldGTGBBhNTCwRwVnJnNCCnbRV
QQpWrpHtrHrpNRRJNtfbJCVR
PvQQFPzccvBglclNscls
NsszMMNGWLcWBhMF
gTtwvbqfnDTdpvqDftpnnntDZvLFQFBLmRWFRhJZJhLLBRQB
DwrpDbngprPWGllSNrSS
nCqdLPZPMMZLNvtGhRmGhGPmtW
TSrVZVSZVwFTgSVtrtchvWRRrtWtcr
gjbjBjgTjfgfVfHHppBLMnqLMDnqClsZJLLD
hrqShCPCpHHBVBGWQFVQGFGnzQDf
tgvZsbwsbcMbRsgccjDGFvGFfWJLLzFFQJ
TTZmMcgmbmWZMctbbtsHrrqqSHHrCrPBBSCPrT
HHHNZLGLpBpRSvWlGlqhPghqDGnnFr
QCNCMTJdjMjdjsQTbdQmmCQDngFqnggPFcPcnPFcDqcbDn
MNJfzNsfJdJjdzwMNjjTJttSHVStRtZVwHvWRWtZHt
DSbvDdDbbwHgCSgZPwpbPgmTTJhsTTChqTJssQssFmJJ
zzjMNNGMMRcNNhvnvFqmtJJv
WWffvlVrcGzGlcjLvfrVRLHgHgpDPSbPpwwHbWbBbPPH
FCCjjFlFtCjzlpTHtJsQTTcpTT
DWLhWSgDWWdSWLwmmpHHQTHcBTBvvwHvHl
mgGRhrLLgWqnjrfCNlzP
cLsslBlsqNNTHlTVNbLZZLRCQbZZdQdpbP
JGfJhhwfwBBSJPRdZddpZRQbfR
hWmWGgDhJrFhBcWsssWHvHll
lmmvlJFtMHFtQzVSRbPGzLJRgG
BcTcrNBrrwwqDBqNqwcrhLpLPVzRhPRPPPgSGVPLbS
TrDqcnsTcsmnvHtdGtMW
CcnDQpSDcnFcPBrmbPQGBsGB
gCtCfRZTBWbjPRbr
qgvHqgJhMfZTtvHgfTghJgMJDpdppFSLLCcSDvLLcdDwvLcw
ffFgGRMWSTGcnDgllDDpDp
dvSdHBrVSLNVLjdlsllcsDqpsZ
SHHHNrLJLvtNQJVvmMfGRGGRCJWJRwzWMh
JNpNDfDBDHVzwHHzpzBWVBPsvsFNCbmbqsFFNsjCmvsmNC
rnnrtLhnrrQZMvtFbWmqtllcFb
GQRdGQLLhMSQhZLZdgdwwHzPDzSVWzVDwJDVpz
LdcGjgdcrMDSFGVfnnGG
HNsCCQFCPvFFBJnnSBJVfDVJwf
HHFRqHPpNppmQPcpLjzrdgtbgztT
GlZZbclGZsDvlGhsShRnCnMQtjtQjnCQsQRM
PggFVcdFNFNNVVFLPdPdrwpWBMMnqMpnttJMnjMnQqtqQtqq
FdgcdcLwfTmSGTmhlbzG
RGvhGrLhhRhlpChZrGSprBdPPHJJSBgSSHqBWBBffH
mQmjmwtTMTVLzHnTPWffPHHJBf
jMmmwMcVcFLFrlRshZbCrF
SnNgNgBlNZSZdZtMrlnSnnQtjpwFwpvFJwFqpwSbqjjqGRpv
CLCcWHLhLTzsDPcCWMLGpFJbGFwsbvGwJwjpRv
zMCTPhmHWzfhQQmndNllNrQg
dbdBdZrQsrdrGslrrSpLvwHmlTmmwScTHv
FgnJqLDLWqNnNpppmpCSSmCJTw
NNhhnRNfzMhgnMDFfGdGLbBVVdQRtPVZGt
BBQJNTTzTcfRhtjhffqDDWCC
vZnsLsVLSvPwPFFnwPlSPgZWqGjChgWCCWWCMCgGMh
srLLnLmlPwrrPwmwwvlRqzQRJBmppQTTQpTdBN
PLDpZGpWbNGWLDfQmsQDwwsmhm
vTzMMbgCfgHQsmQt
VMlRznlzVnTcFzbMcrpJcNrJdjdpZrLdcZ
SftvFcDSvDHsFtctMSvbdjbpqpRRpRTJrMdrrb
QzQZWZnQgQZwBBwsJdqPjdjrnTpJjs
NGmwmgszhZwwGGgZGmggWLVSVHlNVVtDcDltFVVVlVHt
WCfFBfBHHjHHjgHBjJFVcVRwQMbVrRhrJbRRJM
sZMsDqzZPRrRrVswdc
vvTzDzpDTvpDvZPvSnNZZlSHMCHjjFtWmlttlCjmCF
hJZwhrvhBJRrPQPwRRZLllgLqfcqpTggpcTWMTff
HHDzMztbVgTzNpgf
nGbmtjDMFjDjCHbbbHHHdBQQPBrZvJQRwvwRPZQJGs
NNSrMSHRqWpWNNrNMvLffTBBDmsvcmcJLM
lwPPhcddcGPlBDTDlmDvJJsv
bZhzCdPGGFzVVPwVwbNtcqHrpnpZptSZqRrt
GvvSWhmhWBNcBDNc
FzlRRTljjRTjRRmZfbflRTlFFrrMrcBcDVqBVsNDDJsMFr
bttRfzfRHzjlmlnCbTtzbRShgwHGGvppLdpvwLLGLLhd
MHGMWdBFFNsFFHpWSFddMmqVmVBggmlVfbVffjgZml
hcJsTTscvsLDzDJmqVgfqbqnbmfJbJ
PvRTzsPwLcwCprSdwdNW
qfJnJdLpJzrcqCrCzcGfpRSSVBPRSjSSllTNRBdTRS
DbsbtggsbbsghhgvnWWSlVjPSjmmPBtjPNlTmS
vHHHHDHvZHQvWbWsZDgWhDwWzpfGfzfcpFJzczwFJrfffnGC
sQvsRQsFZvfpGhjhQqjpZvjGJHgngPBNHnCBJBCmSBmBNG
HTHwbtdTDDnCTPTT
zwMlVdzbzLzMWvQZRQZfZZlHsR
QhzWwRBPHgFrWWrH
SDgJCCDCsVpMMqTtFpfpqG
gJNCCddSZNSlljQzPPNBzR
dLzVVjfLGCCdRPrdmBtwWttr
NnbNsbTHJnbHbSHlNQsNtwrJRwBMMBhrPJWZRRtM
QSslpFvpSSsQPFCDqqgzcjCj
fcpGshsfNcNZsmRjNqCtnFgbCgHrrggmrn
QvzBlBBQBdJTBzBwVVMgbrwwLFtLtgLFHCHrbF
dlQQMBSSTPZfPcfssZNC
gNGVMzVpVVTdPDWdRdNT
BfjbnCBjBzffHrbrzBDddQWTZZQTTJTQTHHS
zrFncfBjcjnrrlCLwFgpmvFmwGmVLh
MbngccTfWgbWcTTzZghmLshhLRttpthRDLtf
CdFdJHCJjBvBSCNCNJBjjdjpsPDwDtwvptRPmLmzRwhhLR
qJSCCBFHQBFFldrVZZbggnGTzcZQ
lPrpppllcwwpHprppNdfLbQJnWdLJnncdN
tSjjjSSDGgghRbbSTfTbTFTLQn
jCBgDMbBMGghZzCZmmlrrpwp
FhCDFvvPwCjcLhDjhnvjnsdfZTlflQlflLsppdQfld
zPNSmmHrSSHWBNSMMVGzfGfZTZQZzdpdRGZR
WSNVVMMVtHSVbMNWBHqmwcvhcgwgvwtPvgtPDjtw
jSSSjzZMmgSzzwmZBtHcHmtNdncHtnpNcn
VsLsRsJJsTfRVfLRLJlfLlWqNbDcddncvpvbdvcnpqdpdtHq
GQQTsJGGJLlRGJFWffWLhgZwrZBZFZrhtBjrjjZw
ZfzJPvPnLvRJRfZLDfjfrBcqrgsgDBrcrGgslsms
SNhpqSNhpVTNQSMNgWmrlccVGBBmwrBw
QHHFhhNdTNHHfZPFqtqPRtjq
PMZSPSZZGMspsLhLRqRVzfGjvF
tcwwgcgbcbCrtbbtmQQcCqRffFLhRgqjFjRfhFqhqz
CLQtcbcmwmbdrbBrlrCwQTsTPsPsZNBPNWJpZWpTss
lMTrcHrhChWnRzJrznnr
DDJbPwjLJpfBQjPVBpbsGVGVWnZnsqnZsnzsqZ
wJQLbpPJDLfgPbDNCHNlghNCMhcNcl
tlVZhlVWtnBltVtssZBBbPbcpdPwbPWfvcbLvbbb
NwCTFNFDNdSNPpLpfN
CGwRjwDjzFFGRGjjFRjlBtZqMzVVtVqhMZMBZn
HhFdMFHhgrdjcZtZjr
zvvQQvzwzDMjZTjtcrTDtt
BwMwSvQSVlzQlMQzwzNgGHPGGHFCCCgGhsHLCS
zMVtBhhVhhDhtzBtMTTfDrPbmRRmPbQmrQbNQGRQtR
vLlJHgnLpDvHHvHvmPbSQbQRGmJmPRrb
pspwHClCwqplsHqDsMMTFWsWfjzszf
CmmjLwWSWGCHCjwSmStJBgQcccBhwgQtgthQ
WZVFTpqWsMsZpFddzszbVzJQBnRtrQthchdBgtgRtdrc
bbsMTWsMVsZqNZMpqWDqbMsCjvlCfjGCPlLLPLCmSCfCLN
VzsjjVGhpjJrJHCppprt
WtMnqtWdSQDtMRSnLNHHwHwQvrJrJCPN
tTtWSScTddBqdRMmlsbFBfhVBhfjjF
gLMWzdTgLFQHdlMgMRwcwhqqvPcPhVFRDF
tBnGrSCZNZCrtGBsSNGtBPhcgfchqqDPwVPRvNRqwN
CmrgstjZngtBzbjJlQWWHjjM
qttwGWHtVPzJJPqbmb
NrRvfTTghNrpLrrpLTrNrRrhvJmzmzlbbVVbdbdZlDdvzMPb
rfcprNcfgpLrVNnnCcnnscstFGCF
ZZhTfggZsbshGrfshMrNMCSRMMWqCqMNRq
TVTJPDTFccqMCcJw
BBLBmLTLDHFvsQpfgnZhbQvG
ffSrFvVVmVCQSfVDFzDvDDmmnGWCRqGRWNNqlttnRsNtGnWW
PgZQgPJJpTpTHRGtNRGWqZMWWR
wdTdwgbPJTJgTgLSFBbrQvSrFrVS
ppssshsscCVCHhVWVpznnQRBnZnBbzczFPRS
dqqfJGWttfWGlwwPSbFbZnRFPFtFZS
wGwdGdddLfGgMTJfwLMlJMpTCChjHhjTHpjjhmsDHmHW
PZQBhRPQBQrWHFHqHFHCqh
STQSvvvppzSVHJJFWjHC
TTgTvbsbszcNnnvbncvRGPBRtRgQrDPLfftPPR
dMltttpQhmQVZdmhsdrvNCHvlWbHWvHCWrlr
PzzLTGpGPDzFBzqFGFqFvHWHvRJbbrbWvCvjJCLv
BpqTBzpzfGGTTPZtSddtQmVffSst
bwHbRZldhQQfDWWGDjBf
CzvgpsNMsvCvFvpszpnMsFgBTDBDWPnPVJJZZDJnfjDTff
CpcLsFrMZbhRcdmt
HgjpWlhzpWjhWTQPFdPBRQzTMQ
JsfwrqLttwJVLGhRQGGPBd
CttfrqDmDDtCsbZCHjhZHSHNlgcW
QSdCWlCRhWRdlrlZrDssZsGDbv
pjPrpjqFNrZNGnBbsNDG
wjjVHjfLQRCgdLrC
PjMpRdBdjMSGsjpdprqtwCrNGrrNlthhrG
WQzDzLZDgzZcqlqqrtJclJnh
zbWHQHDfDWZHfLZHfffWVZpRSPpdVvBSPMqVMPjdvspS
TMBJLTJlFHBjFFtMGngpvvpgvQmtNSNngv
bVhsZswRCbbVZWVfVZwVSpmSQPPvNHwPHmgmSSNN
dCCVZZcbWVVcCbbfsLrdjFMJdDDHBTMrjr
vNWcTWnCqNCPPjhhHsQrfgszrTJRQsfRQD
wLdwMBLFBBQJpszJBqzB
lLwVmMSmttVMlSNqcbcbSbNcvHbh
PVfJfDWrPVPPLcPPFWcjPrqlqqQsljRpplqBQpRvSQvs
NdggMTCChMgdChNmdtTbtmsQSRhQslhlpFRpFwllwQvw
CdnGzbGbgMGMdTCZZDDJcZFDDnWrPH
fsshhnfLZSvcVbdcZVJj
RCCSmDFFpRqHQDgWvbGjgjDdbG
pFmFtCSBCSMBBLwrPsBPNlNB
fWWcwbbwbWfGCPgPfvbwgvgcQQqQLsGLJQTZHHrZRsrLqlJs
VnszmsDBpMFpzNFlrlRLRRHZqRHr
VpdzDMNzNDjpsdzdnzDcPCwtWCjhbthvfgtgwP
SPQtSWDLLltQQctHLSBSWHlWgFwhMRsgwggrFJPgdgwwGJhJ
nCqmfVqfVjTznCMhsGRRRgGFMffw
mCnNTVzVvjmqNtlDtbttDlBM
LjctjtppFWmgthgs
nBrNvzTqlDJlbbZgvmhMZVZb
JJrnTrrgGDqDPwwSPHPpfjRdPc
sTQmCmmVqmJHSTjGdMMfMNNvNHvc
rFbzlLLWWPzwlWrlbwzrWbRvdfFjdjpvjfFNNMccphCvhN
rrWzrwzPBBBCZTJgZg
gffvjftWddzZtbvdNvgZLwBBMJLSWMDMDDBRWRmS
PqPqpqcCnCpVqlClTQQmPMDwPRJJBLLLhS
VCGqlHmmHsjtHNsZ
mmMlVllWmhmmBzzLGMWlBmpstptPRRZpPMFJSpRsFRFs
DgjnndQcNTCCCDNcdSRSdtZPfwfwJSJJ
gQjCQqQjHNnjDCgHNcZGZLhHzrLVLGzlrGmb
JfwfJpBgJSMphZqtqDDG
QcQrssrGCcMCVcMc
RnljPRnPjWbGRbjnjbvmSzwHfHgwfJHzdLFSwBFW
sBjbHCBCnjvsJCHBsbvwwJGfRNFFFfFGTcrVFffNHRTP
zDqdpqMgMtgzthgDtQmzGPTVSTVrVGTFSVFFqNRF
zDLdphmmLhMhDhQdlzgLLbjCnWswWWlrZJBswCJJZl
FMNrQFgrVwmrpJMwMTMPflbsHPTtlSbftSjCbC
zGnGnhnGzDqRLnZLHNHbbbHDlltNSjCl
nzddcRzzBnRRvRRhvnQFpmpgggJVcVmMQmgN
CCpMlhwwpJpdBlsdcjvtZDFrtmRqmDrsmv
gzVPbjSPfSPTTTPnWVSbbvDQZZDZrFWDvFDvmvQQmZ
LTbLTPgnTzLVPNNGnNTgVNPlGdHHCphMwHMwjMphlpjccl
rMMrqcrmJqJqmCsTPWWGGPzPlPPrGL
fnwqwwZwRnVlWWnzWBWlDP
jvVHvqfpJmhtHJtH
NLMVQjRNTJCTJtZTJc
DlGlGHvFHGDgcFCtfhCJFtZc
DBHGGGSDvGDPHWBGdBbSvgWDNVMjLLRnmNmjPLNPNcRQVnjj
tsGdTJdJtNllzjGRzm
HZvvDLLWqbBBMRMRNjVhHRmn
vZDCvqqgBDZZjbZDrWqBvpdpFpcdpCJcPTSJJtptpP";
