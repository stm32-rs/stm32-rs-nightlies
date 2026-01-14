///Register `L2CACR` reader
pub type R = crate::R<L2CACRrs>;
///Register `L2CACR` writer
pub type W = crate::W<L2CACRrs>;
///Field `CONSTA` reader - CONSTA
pub type CONSTA_R = crate::FieldReader;
///Field `CONSTA` writer - CONSTA
pub type CONSTA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - CONSTA
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CACR")
            .field("consta", &self.consta())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - CONSTA
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W<'_, L2CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
/**This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.

You can [`read`](crate::Reg::read) this register and get [`l2cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#LTDC:L2CACR)*/
pub struct L2CACRrs;
impl crate::RegisterSpec for L2CACRrs {
    type Ux = u32;
}
///`read()` method returns [`l2cacr::R`](R) reader structure
impl crate::Readable for L2CACRrs {}
///`write(|w| ..)` method takes [`l2cacr::W`](W) writer structure
impl crate::Writable for L2CACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CACR to value 0xff
impl crate::Resettable for L2CACRrs {
    const RESET_VALUE: u32 = 0xff;
}
