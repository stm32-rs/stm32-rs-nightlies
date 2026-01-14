///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBGSLEEP` reader - Debug in Sleep mode enable
pub type DBGSLEEP_R = crate::BitReader;
///Field `DBGSLEEP` writer - Debug in Sleep mode enable
pub type DBGSLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTOP` reader - Debug in Stop mode enable
pub type DBGSTOP_R = crate::BitReader;
///Field `DBGSTOP` writer - Debug in Stop mode enable
pub type DBGSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGSTBY` reader - Debug in Standby mode enable
pub type DBGSTBY_R = crate::BitReader;
///Field `DBGSTBY` writer - Debug in Standby mode enable
pub type DBGSTBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRT` reader - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials
pub type DCRT_R = crate::BitReader;
///Field `DCRT` writer - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials
pub type DCRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACECLKEN` reader - Trace port clock enable. This bit enables the trace port clock, TRACECLK.
pub type TRACECLKEN_R = crate::BitReader;
///Field `TRACECLKEN` writer - Trace port clock enable. This bit enables the trace port clock, TRACECLK.
pub type TRACECLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `D1DBGCKEN` reader - D1 debug clock enable This bit allows the debug components in the D1 clock domain (excluding those in the processor core) to be switched off if they are not needed.
pub type D1DBGCKEN_R = crate::BitReader;
///Field `D1DBGCKEN` writer - D1 debug clock enable This bit allows the debug components in the D1 clock domain (excluding those in the processor core) to be switched off if they are not needed.
pub type D1DBGCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRGOEN` reader - External trigger output enable This bit controls the direction of the bi-directional trigger pin, TRGIO.
pub type TRGOEN_R = crate::BitReader;
///Field `TRGOEN` writer - External trigger output enable This bit controls the direction of the bi-directional trigger pin, TRGIO.
pub type TRGOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Debug in Sleep mode enable
    #[inline(always)]
    pub fn dbgsleep(&self) -> DBGSLEEP_R {
        DBGSLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Debug in Stop mode enable
    #[inline(always)]
    pub fn dbgstop(&self) -> DBGSTOP_R {
        DBGSTOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Debug in Standby mode enable
    #[inline(always)]
    pub fn dbgstby(&self) -> DBGSTBY_R {
        DBGSTBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials
    #[inline(always)]
    pub fn dcrt(&self) -> DCRT_R {
        DCRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - Trace port clock enable. This bit enables the trace port clock, TRACECLK.
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - D1 debug clock enable This bit allows the debug components in the D1 clock domain (excluding those in the processor core) to be switched off if they are not needed.
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 28 - External trigger output enable This bit controls the direction of the bi-directional trigger pin, TRGIO.
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbgsleep", &self.dbgsleep())
            .field("dbgstop", &self.dbgstop())
            .field("dbgstby", &self.dbgstby())
            .field("dcrt", &self.dcrt())
            .field("traceclken", &self.traceclken())
            .field("d1dbgcken", &self.d1dbgcken())
            .field("trgoen", &self.trgoen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug in Sleep mode enable
    #[inline(always)]
    pub fn dbgsleep(&mut self) -> DBGSLEEP_W<'_, CRrs> {
        DBGSLEEP_W::new(self, 0)
    }
    ///Bit 1 - Debug in Stop mode enable
    #[inline(always)]
    pub fn dbgstop(&mut self) -> DBGSTOP_W<'_, CRrs> {
        DBGSTOP_W::new(self, 1)
    }
    ///Bit 2 - Debug in Standby mode enable
    #[inline(always)]
    pub fn dbgstby(&mut self) -> DBGSTBY_W<'_, CRrs> {
        DBGSTBY_W::new(self, 2)
    }
    ///Bit 16 - Debug credentials reset type This bit selects which type of reset is used to revoke the debug authentication credentials
    #[inline(always)]
    pub fn dcrt(&mut self) -> DCRT_W<'_, CRrs> {
        DCRT_W::new(self, 16)
    }
    ///Bit 20 - Trace port clock enable. This bit enables the trace port clock, TRACECLK.
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<'_, CRrs> {
        TRACECLKEN_W::new(self, 20)
    }
    ///Bit 21 - D1 debug clock enable This bit allows the debug components in the D1 clock domain (excluding those in the processor core) to be switched off if they are not needed.
    #[inline(always)]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W<'_, CRrs> {
        D1DBGCKEN_W::new(self, 21)
    }
    ///Bit 28 - External trigger output enable This bit controls the direction of the bi-directional trigger pin, TRGIO.
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W<'_, CRrs> {
        TRGOEN_W::new(self, 28)
    }
}
/**DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DBGMCU:CR)*/
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
