///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `CLKWISE_F` reader - Clock Wise Flag:
pub type CLKWISE_F_R = crate::BitReader;
///Field `CLKWISE_F` writer - Clock Wise Flag:
pub type CLKWISE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKWISE_F` reader - Anti Clock Wise Flag:
pub type ACLKWISE_F_R = crate::BitReader;
///Field `ACLKWISE_F` writer - Anti Clock Wise Flag:
pub type ACLKWISE_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP_F` reader - Tamper Flag
pub type TAMP_F_R = crate::BitReader;
///Field `TAMP_F` writer - Tamper Flag
pub type TAMP_F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNT_OFB_F` reader - Out of Bound Counter Flag
pub type CNT_OFB_F_R = crate::BitReader;
///Field `CNT_OFB_F` writer - Out of Bound Counter Flag
pub type CNT_OFB_F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clock Wise Flag:
    #[inline(always)]
    pub fn clkwise_f(&self) -> CLKWISE_F_R {
        CLKWISE_F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Anti Clock Wise Flag:
    #[inline(always)]
    pub fn aclkwise_f(&self) -> ACLKWISE_F_R {
        ACLKWISE_F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper Flag
    #[inline(always)]
    pub fn tamp_f(&self) -> TAMP_F_R {
        TAMP_F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Out of Bound Counter Flag
    #[inline(always)]
    pub fn cnt_ofb_f(&self) -> CNT_OFB_F_R {
        CNT_OFB_F_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("clkwise_f", &self.clkwise_f())
            .field("aclkwise_f", &self.aclkwise_f())
            .field("tamp_f", &self.tamp_f())
            .field("cnt_ofb_f", &self.cnt_ofb_f())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock Wise Flag:
    #[inline(always)]
    pub fn clkwise_f(&mut self) -> CLKWISE_F_W<'_, ISRrs> {
        CLKWISE_F_W::new(self, 0)
    }
    ///Bit 1 - Anti Clock Wise Flag:
    #[inline(always)]
    pub fn aclkwise_f(&mut self) -> ACLKWISE_F_W<'_, ISRrs> {
        ACLKWISE_F_W::new(self, 1)
    }
    ///Bit 2 - Tamper Flag
    #[inline(always)]
    pub fn tamp_f(&mut self) -> TAMP_F_W<'_, ISRrs> {
        TAMP_F_W::new(self, 2)
    }
    ///Bit 3 - Out of Bound Counter Flag
    #[inline(always)]
    pub fn cnt_ofb_f(&mut self) -> CNT_OFB_F_W<'_, ISRrs> {
        CNT_OFB_F_W::new(self, 3)
    }
}
/**LCSC_ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
