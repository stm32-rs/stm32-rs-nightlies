///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `SW` reader - System clock switch
pub type SW_R = crate::FieldReader;
///Field `SW` writer - System clock switch
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWS` reader - System clock switch status
pub type SWS_R = crate::FieldReader;
///Field `HPRE` reader - AHB prescaler
pub type HPRE_R = crate::FieldReader;
///Field `HPRE` writer - AHB prescaler
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PPRE1` reader - PB low-speed prescaler (APB1)
pub type PPRE1_R = crate::FieldReader;
///Field `PPRE1` writer - PB low-speed prescaler (APB1)
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PPRE2` reader - APB high-speed prescaler (APB2)
pub type PPRE2_R = crate::FieldReader;
///Field `PPRE2` writer - APB high-speed prescaler (APB2)
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `STOPWUCK` reader - Wakeup from Stop and CSS backup clock selection
pub type STOPWUCK_R = crate::BitReader;
///Field `STOPWUCK` writer - Wakeup from Stop and CSS backup clock selection
pub type STOPWUCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCOSEL` reader - Microcontroller clock output
pub type MCOSEL_R = crate::FieldReader;
///Field `MCOSEL` writer - Microcontroller clock output
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCOPRE` reader - Microcontroller clock output prescaler
pub type MCOPRE_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - PB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 15 - Wakeup from Stop and CSS backup clock selection
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 24:26 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("mcopre", &self.mcopre())
            .field("mcosel", &self.mcosel())
            .field("stopwuck", &self.stopwuck())
            .field("ppre2", &self.ppre2())
            .field("ppre1", &self.ppre1())
            .field("hpre", &self.hpre())
            .field("sws", &self.sws())
            .field("sw", &self.sw())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, CFGRrs> {
        HPRE_W::new(self, 4)
    }
    ///Bits 8:10 - PB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<'_, CFGRrs> {
        PPRE1_W::new(self, 8)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<'_, CFGRrs> {
        PPRE2_W::new(self, 11)
    }
    ///Bit 15 - Wakeup from Stop and CSS backup clock selection
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<'_, CFGRrs> {
        STOPWUCK_W::new(self, 15)
    }
    ///Bits 24:26 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
}
/**Clock configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#RCC:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
