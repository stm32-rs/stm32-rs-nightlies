///Register `APBCFGR` reader
pub type R = crate::R<APBCFGRrs>;
///Register `APBCFGR` writer
pub type W = crate::W<APBCFGRrs>;
///Field `PPRE1` reader - CPU domain APB1 prescaler
pub type PPRE1_R = crate::FieldReader;
///Field `PPRE1` writer - CPU domain APB1 prescaler
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE2` reader - CPU domain APB2 prescaler
pub type PPRE2_R = crate::FieldReader;
///Field `PPRE2` writer - CPU domain APB2 prescaler
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE4` reader - CPU domain APB4 prescaler
pub type PPRE4_R = crate::FieldReader;
///Field `PPRE4` writer - CPU domain APB4 prescaler
pub type PPRE4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE5` reader - CPU domain APB5 prescaler
pub type PPRE5_R = crate::FieldReader;
///Field `PPRE5` writer - CPU domain APB5 prescaler
pub type PPRE5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - CPU domain APB1 prescaler
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - CPU domain APB2 prescaler
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - CPU domain APB4 prescaler
    #[inline(always)]
    pub fn ppre4(&self) -> PPRE4_R {
        PPRE4_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - CPU domain APB5 prescaler
    #[inline(always)]
    pub fn ppre5(&self) -> PPRE5_R {
        PPRE5_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBCFGR")
            .field("ppre1", &self.ppre1())
            .field("ppre2", &self.ppre2())
            .field("ppre4", &self.ppre4())
            .field("ppre5", &self.ppre5())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - CPU domain APB1 prescaler
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, APBCFGRrs> {
        PPRE1_W::new(self, 0)
    }
    ///Bits 4:6 - CPU domain APB2 prescaler
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, APBCFGRrs> {
        PPRE2_W::new(self, 4)
    }
    ///Bits 8:10 - CPU domain APB4 prescaler
    #[inline(always)]
    pub fn ppre4(&mut self) -> PPRE4_W<'_, APBCFGRrs> {
        PPRE4_W::new(self, 8)
    }
    ///Bits 12:14 - CPU domain APB5 prescaler
    #[inline(always)]
    pub fn ppre5(&mut self) -> PPRE5_W<'_, APBCFGRrs> {
        PPRE5_W::new(self, 12)
    }
}
/**RCC APB clocks configuration register

You can [`read`](crate::Reg::read) this register and get [`apbcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RCC:APBCFGR)*/
pub struct APBCFGRrs;
impl crate::RegisterSpec for APBCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`apbcfgr::R`](R) reader structure
impl crate::Readable for APBCFGRrs {}
///`write(|w| ..)` method takes [`apbcfgr::W`](W) writer structure
impl crate::Writable for APBCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBCFGR to value 0
impl crate::Resettable for APBCFGRrs {}
