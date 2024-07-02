///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
/**Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0W {
    ///1: Sets the corresponding ODx bit
    Set = 1,
}
impl From<BS0W> for bool {
    #[inline(always)]
    fn from(variant: BS0W) -> Self {
        variant as u8 != 0
    }
}
///Field `BS0` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type BS0_W<'a, REG> = crate::BitWriter<'a, REG, BS0W>;
impl<'a, REG> BS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Sets the corresponding ODx bit
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BS0W::Set)
    }
}
///Field `BS1` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS1_W;
///Field `BS2` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS2_W;
///Field `BS3` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS3_W;
///Field `BS4` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS4_W;
///Field `BS5` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS5_W;
///Field `BS6` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS6_W;
///Field `BS7` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS7_W;
///Field `BS8` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS8_W;
///Field `BS9` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS9_W;
///Field `BS10` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS10_W;
///Field `BS11` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS11_W;
///Field `BS12` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS12_W;
///Field `BS13` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS13_W;
///Field `BS14` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS14_W;
///Field `BS15` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS15_W;
/**Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W {
    ///1: Resets the corresponding ODx bit
    Reset = 1,
}
impl From<BR0W> for bool {
    #[inline(always)]
    fn from(variant: BR0W) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type BR0_W<'a, REG> = crate::BitWriter<'a, REG, BR0W>;
impl<'a, REG> BR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Resets the corresponding ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR0W::Reset)
    }
}
///Field `BR1` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR1_W;
///Field `BR2` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR2_W;
///Field `BR3` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR3_W;
///Field `BR4` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR4_W;
///Field `BR5` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR5_W;
///Field `BR6` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR6_W;
///Field `BR7` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR7_W;
///Field `BR8` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR8_W;
///Field `BR9` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR9_W;
///Field `BR10` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR10_W;
///Field `BR11` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR11_W;
///Field `BR12` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR12_W;
///Field `BR13` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR13_W;
///Field `BR14` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR14_W;
///Field `BR15` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR15_W;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<BSRRrs> {
        BS0_W::new(self, 0)
    }
    ///Bit 1 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<BSRRrs> {
        BS1_W::new(self, 1)
    }
    ///Bit 2 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<BSRRrs> {
        BS2_W::new(self, 2)
    }
    ///Bit 3 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<BSRRrs> {
        BS3_W::new(self, 3)
    }
    ///Bit 4 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<BSRRrs> {
        BS4_W::new(self, 4)
    }
    ///Bit 5 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<BSRRrs> {
        BS5_W::new(self, 5)
    }
    ///Bit 6 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<BSRRrs> {
        BS6_W::new(self, 6)
    }
    ///Bit 7 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<BSRRrs> {
        BS7_W::new(self, 7)
    }
    ///Bit 8 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<BSRRrs> {
        BS8_W::new(self, 8)
    }
    ///Bit 9 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS9_W<BSRRrs> {
        BS9_W::new(self, 9)
    }
    ///Bit 10 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS10_W<BSRRrs> {
        BS10_W::new(self, 10)
    }
    ///Bit 11 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS11_W<BSRRrs> {
        BS11_W::new(self, 11)
    }
    ///Bit 12 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS12_W<BSRRrs> {
        BS12_W::new(self, 12)
    }
    ///Bit 13 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS13_W<BSRRrs> {
        BS13_W::new(self, 13)
    }
    ///Bit 14 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS14_W<BSRRrs> {
        BS14_W::new(self, 14)
    }
    ///Bit 15 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS15_W<BSRRrs> {
        BS15_W::new(self, 15)
    }
    ///Bit 16 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<BSRRrs> {
        BR0_W::new(self, 16)
    }
    ///Bit 17 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<BSRRrs> {
        BR1_W::new(self, 17)
    }
    ///Bit 18 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<BSRRrs> {
        BR2_W::new(self, 18)
    }
    ///Bit 19 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BSRRrs> {
        BR3_W::new(self, 19)
    }
    ///Bit 20 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<BSRRrs> {
        BR4_W::new(self, 20)
    }
    ///Bit 21 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<BSRRrs> {
        BR5_W::new(self, 21)
    }
    ///Bit 22 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<BSRRrs> {
        BR6_W::new(self, 22)
    }
    ///Bit 23 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<BSRRrs> {
        BR7_W::new(self, 23)
    }
    ///Bit 24 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<BSRRrs> {
        BR8_W::new(self, 24)
    }
    ///Bit 25 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<BSRRrs> {
        BR9_W::new(self, 25)
    }
    ///Bit 26 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<BSRRrs> {
        BR10_W::new(self, 26)
    }
    ///Bit 27 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<BSRRrs> {
        BR11_W::new(self, 27)
    }
    ///Bit 28 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<BSRRrs> {
        BR12_W::new(self, 28)
    }
    ///Bit 29 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<BSRRrs> {
        BR13_W::new(self, 29)
    }
    ///Bit 30 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<BSRRrs> {
        BR14_W::new(self, 30)
    }
    ///Bit 31 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<BSRRrs> {
        BR15_W::new(self, 31)
    }
}
/**GPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#GPIOI:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {
    const RESET_VALUE: u32 = 0;
}
