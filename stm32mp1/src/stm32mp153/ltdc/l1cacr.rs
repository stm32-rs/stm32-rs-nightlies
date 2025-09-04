///Register `L1CACR` reader
pub type R = crate::R<L1CACRrs>;
///Register `L1CACR` writer
pub type W = crate::W<L1CACRrs>;
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
        f.debug_struct("L1CACR")
            .field("consta", &self.consta())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - CONSTA
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W<L1CACRrs> {
        CONSTA_W::new(self, 0)
    }
}
/**This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.

You can [`read`](crate::Reg::read) this register and get [`l1cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CACR)*/
pub struct L1CACRrs;
impl crate::RegisterSpec for L1CACRrs {
    type Ux = u32;
}
///`read()` method returns [`l1cacr::R`](R) reader structure
impl crate::Readable for L1CACRrs {}
///`write(|w| ..)` method takes [`l1cacr::W`](W) writer structure
impl crate::Writable for L1CACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1CACR to value 0xff
impl crate::Resettable for L1CACRrs {
    const RESET_VALUE: u32 = 0xff;
}
