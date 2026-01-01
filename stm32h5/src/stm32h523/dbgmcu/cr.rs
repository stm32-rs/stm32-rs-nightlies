///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBG_STOP` reader - Allows debug in Stop mode
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - Allows debug in Stop mode
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STANDBY` reader - Allows debug in Standby mode
pub type DBG_STANDBY_R = crate::BitReader;
///Field `DBG_STANDBY` writer - Allows debug in Standby mode
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACE_IOEN` reader - trace pin enable
pub type TRACE_IOEN_R = crate::BitReader;
///Field `TRACE_IOEN` writer - trace pin enable
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACE_EN` reader - trace port and clock enable.
pub type TRACE_EN_R = crate::BitReader;
///Field `TRACE_EN` writer - trace port and clock enable.
pub type TRACE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACE_MODE` reader - trace pin assignment
pub type TRACE_MODE_R = crate::FieldReader;
///Field `TRACE_MODE` writer - trace pin assignment
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DCRT` reader - Debug credentials reset type
pub type DCRT_R = crate::BitReader;
///Field `DCRT` writer - Debug credentials reset type
pub type DCRT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Allows debug in Stop mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allows debug in Standby mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - trace pin enable
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - trace port and clock enable.
    #[inline(always)]
    pub fn trace_en(&self) -> TRACE_EN_R {
        TRACE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - trace pin assignment
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 16 - Debug credentials reset type
    #[inline(always)]
    pub fn dcrt(&self) -> DCRT_R {
        DCRT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("trace_ioen", &self.trace_ioen())
            .field("trace_en", &self.trace_en())
            .field("trace_mode", &self.trace_mode())
            .field("dcrt", &self.dcrt())
            .finish()
    }
}
impl W {
    ///Bit 1 - Allows debug in Stop mode
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<'_, CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - Allows debug in Standby mode
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<'_, CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    ///Bit 4 - trace pin enable
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<'_, CRrs> {
        TRACE_IOEN_W::new(self, 4)
    }
    ///Bit 5 - trace port and clock enable.
    #[inline(always)]
    pub fn trace_en(&mut self) -> TRACE_EN_W<'_, CRrs> {
        TRACE_EN_W::new(self, 5)
    }
    ///Bits 6:7 - trace pin assignment
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<'_, CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
    ///Bit 16 - Debug credentials reset type
    #[inline(always)]
    pub fn dcrt(&mut self) -> DCRT_W<'_, CRrs> {
        DCRT_W::new(self, 16)
    }
}
/**DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DBGMCU:CR)*/
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
