///Register `CSR1` reader
pub type R = crate::R<CSR1rs>;
///Field `PVDO` reader - PVDO
pub type PVDO_R = crate::BitReader;
///Field `AVDO` reader - AVDO
pub type AVDO_R = crate::BitReader;
impl R {
    ///Bit 4 - PVDO
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - AVDO
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR1")
            .field("pvdo", &self.pvdo())
            .field("avdo", &self.avdo())
            .finish()
    }
}
/**Reset on any system reset.

You can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#PWR:CSR1)*/
pub struct CSR1rs;
impl crate::RegisterSpec for CSR1rs {
    type Ux = u32;
}
///`read()` method returns [`csr1::R`](R) reader structure
impl crate::Readable for CSR1rs {}
///`reset()` method sets CSR1 to value 0
impl crate::Resettable for CSR1rs {}
