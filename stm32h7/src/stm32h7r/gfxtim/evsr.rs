///Register `EVSR` reader
pub type R = crate::R<EVSRrs>;
///Register `EVSR` writer
pub type W = crate::W<EVSRrs>;
///Field `LES1` reader - line-event selection 1 This field defines the line-event selection for complex event 1 generation. others: reserved
pub type LES1_R = crate::FieldReader;
///Field `LES1` writer - line-event selection 1 This field defines the line-event selection for complex event 1 generation. others: reserved
pub type LES1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FES1` reader - frame-event selection 1 This field defines the frame-event selection for complex event 1 generation. others: reserved
pub type FES1_R = crate::FieldReader;
///Field `FES1` writer - frame-event selection 1 This field defines the frame-event selection for complex event 1 generation. others: reserved
pub type FES1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LES2` reader - line-event selection 2 This field defines the line-event selection for complex event 2 generation. others: reserved
pub type LES2_R = crate::FieldReader;
///Field `LES2` writer - line-event selection 2 This field defines the line-event selection for complex event 2 generation. others: reserved
pub type LES2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FES2` reader - frame-event selection 2 This field defines the frame-event selection for complex event 2 generation. others: reserved
pub type FES2_R = crate::FieldReader;
///Field `FES2` writer - frame-event selection 2 This field defines the frame-event selection for complex event 2 generation. others: reserved
pub type FES2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LES3` reader - line-event selection 3 This field defines the line-event selection for complex event 3 generation. others: reserved
pub type LES3_R = crate::FieldReader;
///Field `LES3` writer - line-event selection 3 This field defines the line-event selection for complex event 3 generation. others: reserved
pub type LES3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FES3` reader - frame-event selection 3 This field defines the frame-event selection for complex event 3 generation. others: reserved
pub type FES3_R = crate::FieldReader;
///Field `FES3` writer - frame-event selection 3 This field defines the frame-event selection for complex event 3 generation. others: reserved
pub type FES3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LES4` reader - line-event selection 4 This field defines the line-event selection for complex event 4 generation. others: Reserved
pub type LES4_R = crate::FieldReader;
///Field `LES4` writer - line-event selection 4 This field defines the line-event selection for complex event 4 generation. others: Reserved
pub type LES4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FES4` reader - frame-event selection 4 This field defines the frame-event selection for complex event 4 generation. others: reserved
pub type FES4_R = crate::FieldReader;
///Field `FES4` writer - frame-event selection 4 This field defines the frame-event selection for complex event 4 generation. others: reserved
pub type FES4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - line-event selection 1 This field defines the line-event selection for complex event 1 generation. others: reserved
    #[inline(always)]
    pub fn les1(&self) -> LES1_R {
        LES1_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - frame-event selection 1 This field defines the frame-event selection for complex event 1 generation. others: reserved
    #[inline(always)]
    pub fn fes1(&self) -> FES1_R {
        FES1_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - line-event selection 2 This field defines the line-event selection for complex event 2 generation. others: reserved
    #[inline(always)]
    pub fn les2(&self) -> LES2_R {
        LES2_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - frame-event selection 2 This field defines the frame-event selection for complex event 2 generation. others: reserved
    #[inline(always)]
    pub fn fes2(&self) -> FES2_R {
        FES2_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:18 - line-event selection 3 This field defines the line-event selection for complex event 3 generation. others: reserved
    #[inline(always)]
    pub fn les3(&self) -> LES3_R {
        LES3_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - frame-event selection 3 This field defines the frame-event selection for complex event 3 generation. others: reserved
    #[inline(always)]
    pub fn fes3(&self) -> FES3_R {
        FES3_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - line-event selection 4 This field defines the line-event selection for complex event 4 generation. others: Reserved
    #[inline(always)]
    pub fn les4(&self) -> LES4_R {
        LES4_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - frame-event selection 4 This field defines the frame-event selection for complex event 4 generation. others: reserved
    #[inline(always)]
    pub fn fes4(&self) -> FES4_R {
        FES4_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSR")
            .field("les1", &self.les1())
            .field("fes1", &self.fes1())
            .field("les2", &self.les2())
            .field("fes2", &self.fes2())
            .field("les3", &self.les3())
            .field("fes3", &self.fes3())
            .field("les4", &self.les4())
            .field("fes4", &self.fes4())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - line-event selection 1 This field defines the line-event selection for complex event 1 generation. others: reserved
    #[inline(always)]
    pub fn les1(&mut self) -> LES1_W<'_, EVSRrs> {
        LES1_W::new(self, 0)
    }
    ///Bits 4:6 - frame-event selection 1 This field defines the frame-event selection for complex event 1 generation. others: reserved
    #[inline(always)]
    pub fn fes1(&mut self) -> FES1_W<'_, EVSRrs> {
        FES1_W::new(self, 4)
    }
    ///Bits 8:10 - line-event selection 2 This field defines the line-event selection for complex event 2 generation. others: reserved
    #[inline(always)]
    pub fn les2(&mut self) -> LES2_W<'_, EVSRrs> {
        LES2_W::new(self, 8)
    }
    ///Bits 12:14 - frame-event selection 2 This field defines the frame-event selection for complex event 2 generation. others: reserved
    #[inline(always)]
    pub fn fes2(&mut self) -> FES2_W<'_, EVSRrs> {
        FES2_W::new(self, 12)
    }
    ///Bits 16:18 - line-event selection 3 This field defines the line-event selection for complex event 3 generation. others: reserved
    #[inline(always)]
    pub fn les3(&mut self) -> LES3_W<'_, EVSRrs> {
        LES3_W::new(self, 16)
    }
    ///Bits 20:22 - frame-event selection 3 This field defines the frame-event selection for complex event 3 generation. others: reserved
    #[inline(always)]
    pub fn fes3(&mut self) -> FES3_W<'_, EVSRrs> {
        FES3_W::new(self, 20)
    }
    ///Bits 24:26 - line-event selection 4 This field defines the line-event selection for complex event 4 generation. others: Reserved
    #[inline(always)]
    pub fn les4(&mut self) -> LES4_W<'_, EVSRrs> {
        LES4_W::new(self, 24)
    }
    ///Bits 28:30 - frame-event selection 4 This field defines the frame-event selection for complex event 4 generation. others: reserved
    #[inline(always)]
    pub fn fes4(&mut self) -> FES4_W<'_, EVSRrs> {
        FES4_W::new(self, 28)
    }
}
/**GFXTIM events selection register

You can [`read`](crate::Reg::read) this register and get [`evsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GFXTIM:EVSR)*/
pub struct EVSRrs;
impl crate::RegisterSpec for EVSRrs {
    type Ux = u32;
}
///`read()` method returns [`evsr::R`](R) reader structure
impl crate::Readable for EVSRrs {}
///`write(|w| ..)` method takes [`evsr::W`](W) writer structure
impl crate::Writable for EVSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EVSR to value 0
impl crate::Resettable for EVSRrs {}
