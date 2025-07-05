///Register `AESLEPRIVSTATREG` reader
pub type R = crate::R<AESLEPRIVSTATREGrs>;
///Field `BUSY` reader - AES Le privacy busy status
pub type BUSY_R = crate::BitReader;
///Field `KEYFND` reader - AES Le privacy key finding status
pub type KEYFND_R = crate::BitReader;
///Field `KEYFNDINDEX` reader - AES Le privacy index of the key found in the resolution key list.
pub type KEYFNDINDEX_R = crate::FieldReader;
impl R {
    ///Bit 0 - AES Le privacy busy status
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES Le privacy key finding status
    #[inline(always)]
    pub fn keyfnd(&self) -> KEYFND_R {
        KEYFND_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:9 - AES Le privacy index of the key found in the resolution key list.
    #[inline(always)]
    pub fn keyfndindex(&self) -> KEYFNDINDEX_R {
        KEYFNDINDEX_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESLEPRIVSTATREG")
            .field("busy", &self.busy())
            .field("keyfnd", &self.keyfnd())
            .field("keyfndindex", &self.keyfndindex())
            .finish()
    }
}
/**AESLEPRIVSTATREG register

You can [`read`](crate::Reg::read) this register and get [`aesleprivstatreg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:AESLEPRIVSTATREG)*/
pub struct AESLEPRIVSTATREGrs;
impl crate::RegisterSpec for AESLEPRIVSTATREGrs {
    type Ux = u32;
}
///`read()` method returns [`aesleprivstatreg::R`](R) reader structure
impl crate::Readable for AESLEPRIVSTATREGrs {}
///`reset()` method sets AESLEPRIVSTATREG to value 0
impl crate::Resettable for AESLEPRIVSTATREGrs {}
