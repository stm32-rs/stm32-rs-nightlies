///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `B0OIE` reader - Buffer 0 overflow interrupt enable
pub type B0OIE_R = crate::BitReader;
///Field `B0OIE` writer - Buffer 0 overflow interrupt enable
pub type B0OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B1OIE` reader - Buffer 1 overflow interrupt enable
pub type B1OIE_R = crate::BitReader;
///Field `B1OIE` writer - Buffer 1 overflow interrupt enable
pub type B1OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2OIE` reader - Buffer 2 overflow interrupt enable
pub type B2OIE_R = crate::BitReader;
///Field `B2OIE` writer - Buffer 2 overflow interrupt enable
pub type B2OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B3OIE` reader - Buffer 3 overflow interrupt enable
pub type B3OIE_R = crate::BitReader;
///Field `B3OIE` writer - Buffer 3 overflow interrupt enable
pub type B3OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AMEIE` reader - AXI master error interrupt enable
pub type AMEIE_R = crate::BitReader;
///Field `AMEIE` writer - AXI master error interrupt enable
pub type AMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BS` reader - Block size
pub type BS_R = crate::BitReader;
///Field `BS` writer - Block size
pub type BS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ATE` reader - Address translation enable
pub type ATE_R = crate::BitReader;
///Field `ATE` writer - Address translation enable
pub type ATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B0PE` reader - Buffer 0 packing enable
pub type B0PE_R = crate::BitReader;
///Field `B0PE` writer - Buffer 0 packing enable
pub type B0PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B0PM` reader - Buffer 0 packing mode
pub type B0PM_R = crate::BitReader;
///Field `B0PM` writer - Buffer 0 packing mode
pub type B0PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B1PE` reader - Buffer 1 packing enable
pub type B1PE_R = crate::BitReader;
///Field `B1PE` writer - Buffer 1 packing enable
pub type B1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B1PM` reader - Buffer 1 packing mode
pub type B1PM_R = crate::BitReader;
///Field `B1PM` writer - Buffer 1 packing mode
pub type B1PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2PE` reader - Buffer 2 packing enable
pub type B2PE_R = crate::BitReader;
///Field `B2PE` writer - Buffer 2 packing enable
pub type B2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2PM` reader - Buffer 2 packing mode
pub type B2PM_R = crate::BitReader;
///Field `B2PM` writer - Buffer 2 packing mode
pub type B2PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B3PE` reader - Buffer 3 packing enable
pub type B3PE_R = crate::BitReader;
///Field `B3PE` writer - Buffer 3 packing enable
pub type B3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B3PM` reader - Buffer 3 packing mode
pub type B3PM_R = crate::BitReader;
///Field `B3PM` writer - Buffer 3 packing mode
pub type B3PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Buffer 0 overflow interrupt enable
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AXI master error interrupt enable
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Block size
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 15 - Address translation enable
    #[inline(always)]
    pub fn ate(&self) -> ATE_R {
        ATE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - Buffer 0 packing enable
    #[inline(always)]
    pub fn b0pe(&self) -> B0PE_R {
        B0PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Buffer 0 packing mode
    #[inline(always)]
    pub fn b0pm(&self) -> B0PM_R {
        B0PM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Buffer 1 packing enable
    #[inline(always)]
    pub fn b1pe(&self) -> B1PE_R {
        B1PE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Buffer 1 packing mode
    #[inline(always)]
    pub fn b1pm(&self) -> B1PM_R {
        B1PM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Buffer 2 packing enable
    #[inline(always)]
    pub fn b2pe(&self) -> B2PE_R {
        B2PE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Buffer 2 packing mode
    #[inline(always)]
    pub fn b2pm(&self) -> B2PM_R {
        B2PM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Buffer 3 packing enable
    #[inline(always)]
    pub fn b3pe(&self) -> B3PE_R {
        B3PE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Buffer 3 packing mode
    #[inline(always)]
    pub fn b3pm(&self) -> B3PM_R {
        B3PM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("b0oie", &self.b0oie())
            .field("b1oie", &self.b1oie())
            .field("b2oie", &self.b2oie())
            .field("b3oie", &self.b3oie())
            .field("ameie", &self.ameie())
            .field("bs", &self.bs())
            .field("ate", &self.ate())
            .field("b0pe", &self.b0pe())
            .field("b0pm", &self.b0pm())
            .field("b1pe", &self.b1pe())
            .field("b1pm", &self.b1pm())
            .field("b2pe", &self.b2pe())
            .field("b2pm", &self.b2pm())
            .field("b3pe", &self.b3pe())
            .field("b3pm", &self.b3pm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Buffer 0 overflow interrupt enable
    #[inline(always)]
    pub fn b0oie(&mut self) -> B0OIE_W<'_, CRrs> {
        B0OIE_W::new(self, 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable
    #[inline(always)]
    pub fn b1oie(&mut self) -> B1OIE_W<'_, CRrs> {
        B1OIE_W::new(self, 1)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable
    #[inline(always)]
    pub fn b2oie(&mut self) -> B2OIE_W<'_, CRrs> {
        B2OIE_W::new(self, 2)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable
    #[inline(always)]
    pub fn b3oie(&mut self) -> B3OIE_W<'_, CRrs> {
        B3OIE_W::new(self, 3)
    }
    ///Bit 4 - AXI master error interrupt enable
    #[inline(always)]
    pub fn ameie(&mut self) -> AMEIE_W<'_, CRrs> {
        AMEIE_W::new(self, 4)
    }
    ///Bit 6 - Block size
    #[inline(always)]
    pub fn bs(&mut self) -> BS_W<'_, CRrs> {
        BS_W::new(self, 6)
    }
    ///Bit 15 - Address translation enable
    #[inline(always)]
    pub fn ate(&mut self) -> ATE_W<'_, CRrs> {
        ATE_W::new(self, 15)
    }
    ///Bit 24 - Buffer 0 packing enable
    #[inline(always)]
    pub fn b0pe(&mut self) -> B0PE_W<'_, CRrs> {
        B0PE_W::new(self, 24)
    }
    ///Bit 25 - Buffer 0 packing mode
    #[inline(always)]
    pub fn b0pm(&mut self) -> B0PM_W<'_, CRrs> {
        B0PM_W::new(self, 25)
    }
    ///Bit 26 - Buffer 1 packing enable
    #[inline(always)]
    pub fn b1pe(&mut self) -> B1PE_W<'_, CRrs> {
        B1PE_W::new(self, 26)
    }
    ///Bit 27 - Buffer 1 packing mode
    #[inline(always)]
    pub fn b1pm(&mut self) -> B1PM_W<'_, CRrs> {
        B1PM_W::new(self, 27)
    }
    ///Bit 28 - Buffer 2 packing enable
    #[inline(always)]
    pub fn b2pe(&mut self) -> B2PE_W<'_, CRrs> {
        B2PE_W::new(self, 28)
    }
    ///Bit 29 - Buffer 2 packing mode
    #[inline(always)]
    pub fn b2pm(&mut self) -> B2PM_W<'_, CRrs> {
        B2PM_W::new(self, 29)
    }
    ///Bit 30 - Buffer 3 packing enable
    #[inline(always)]
    pub fn b3pe(&mut self) -> B3PE_W<'_, CRrs> {
        B3PE_W::new(self, 30)
    }
    ///Bit 31 - Buffer 3 packing mode
    #[inline(always)]
    pub fn b3pm(&mut self) -> B3PM_W<'_, CRrs> {
        B3PM_W::new(self, 31)
    }
}
/**GFXMMU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXMMU:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
