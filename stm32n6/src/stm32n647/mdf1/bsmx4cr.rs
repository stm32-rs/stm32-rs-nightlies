///Register `BSMX4CR` reader
pub type R = crate::R<BSMX4CRrs>;
///Register `BSMX4CR` writer
pub type W = crate::W<BSMX4CRrs>;
///Field `BSSEL` reader - Bitstream Selection
pub type BSSEL_R = crate::FieldReader;
///Field `BSSEL` writer - Bitstream Selection
pub type BSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BSMXACTIVE` reader - BSMX active flag
pub type BSMXACTIVE_R = crate::BitReader;
impl R {
    ///Bits 0:4 - Bitstream Selection
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
        f.debug_struct("BSMX4CR")
            .field("bssel", &self.bssel())
            .field("bsmxactive", &self.bsmxactive())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Bitstream Selection
    #[inline(always)]
    pub fn bssel(&mut self) -> BSSEL_W<BSMX4CRrs> {
        BSSEL_W::new(self, 0)
    }
}
/**MDF bitstream matrix control register 4

You can [`read`](crate::Reg::read) this register and get [`bsmx4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#MDF1:BSMX4CR)*/
pub struct BSMX4CRrs;
impl crate::RegisterSpec for BSMX4CRrs {
    type Ux = u32;
}
///`read()` method returns [`bsmx4cr::R`](R) reader structure
impl crate::Readable for BSMX4CRrs {}
///`write(|w| ..)` method takes [`bsmx4cr::W`](W) writer structure
impl crate::Writable for BSMX4CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSMX4CR to value 0
impl crate::Resettable for BSMX4CRrs {}
