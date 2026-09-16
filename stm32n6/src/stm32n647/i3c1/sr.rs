///Register `SR` reader
pub type R = crate::R<SRrs>;
///Field `XDCNT` reader - Data counter
pub type XDCNT_R = crate::FieldReader<u16>;
///Field `ABT` reader - A private read message is ended prematurely by the target (when the I3C acts as controller)
pub type ABT_R = crate::BitReader;
///Field `DIR` reader - Message direction
pub type DIR_R = crate::BitReader;
///Field `MID` reader - Message identifier/counter of a given frame (when the I3C acts as controller)
pub type MID_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Data counter
    #[inline(always)]
    pub fn xdcnt(&self) -> XDCNT_R {
        XDCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 17 - A private read message is ended prematurely by the target (when the I3C acts as controller)
    #[inline(always)]
    pub fn abt(&self) -> ABT_R {
        ABT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Message direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 24:31 - Message identifier/counter of a given frame (when the I3C acts as controller)
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("xdcnt", &self.xdcnt())
            .field("abt", &self.abt())
            .field("dir", &self.dir())
            .field("mid", &self.mid())
            .finish()
    }
}
/**I3C status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#I3C1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
