///Register `ADF_BSMX0CR` reader
pub type R = crate::R<ADF_BSMX0CRrs>;
///Register `ADF_BSMX0CR` writer
pub type W = crate::W<ADF_BSMX0CRrs>;
///Field `BSSEL` reader - Bitstream selection
pub type BSSEL_R = crate::FieldReader;
///Field `BSSEL` writer - Bitstream selection
pub type BSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BSMXACTIVE` reader - BSMX active flag
pub type BSMXACTIVE_R = crate::BitReader;
///Field `BSMXACTIVE` writer - BSMX active flag
pub type BSMXACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - Bitstream selection
    #[inline(always)]
    pub fn bssel(&self) -> BSSEL_R {
        BSSEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 31 - BSMX active flag
    #[inline(always)]
    pub fn bsmxactive(&self) -> BSMXACTIVE_R {
        BSMXACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADF_BSMX0CR")
            .field("bsmxactive", &self.bsmxactive())
            .field("bssel", &self.bssel())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Bitstream selection
    #[inline(always)]
    #[must_use]
    pub fn bssel(&mut self) -> BSSEL_W<ADF_BSMX0CRrs> {
        BSSEL_W::new(self, 0)
    }
    ///Bit 31 - BSMX active flag
    #[inline(always)]
    #[must_use]
    pub fn bsmxactive(&mut self) -> BSMXACTIVE_W<ADF_BSMX0CRrs> {
        BSMXACTIVE_W::new(self, 31)
    }
}
/**ADF bitstream matrix control register 0

You can [`read`](crate::Reg::read) this register and get [`adf_bsmx0cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adf_bsmx0cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#ADF1:ADF_BSMX0CR)*/
pub struct ADF_BSMX0CRrs;
impl crate::RegisterSpec for ADF_BSMX0CRrs {
    type Ux = u32;
}
///`read()` method returns [`adf_bsmx0cr::R`](R) reader structure
impl crate::Readable for ADF_BSMX0CRrs {}
///`write(|w| ..)` method takes [`adf_bsmx0cr::W`](W) writer structure
impl crate::Writable for ADF_BSMX0CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADF_BSMX0CR to value 0
impl crate::Resettable for ADF_BSMX0CRrs {
    const RESET_VALUE: u32 = 0;
}
