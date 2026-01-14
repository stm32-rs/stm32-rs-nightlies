///Register `LB1CFGR` reader
pub type R = crate::R<LB1CFGRrs>;
///Register `LB1CFGR` writer
pub type W = crate::W<LB1CFGRrs>;
///Field `BYTECNT` reader - Byte counter
pub type BYTECNT_R = crate::FieldReader<u16>;
///Field `BYTECNT` writer - Byte counter
pub type BYTECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `LINECNT` reader - Line counter
pub type LINECNT_R = crate::FieldReader<u16>;
///Field `LINECNT` writer - Line counter
pub type LINECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Byte counter
    #[inline(always)]
    pub fn bytecnt(&self) -> BYTECNT_R {
        BYTECNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Line counter
    #[inline(always)]
    pub fn linecnt(&self) -> LINECNT_R {
        LINECNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LB1CFGR")
            .field("bytecnt", &self.bytecnt())
            .field("linecnt", &self.linecnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Byte counter
    #[inline(always)]
    pub fn bytecnt(&mut self) -> BYTECNT_W<'_, LB1CFGRrs> {
        BYTECNT_W::new(self, 0)
    }
    ///Bits 16:31 - Line counter
    #[inline(always)]
    pub fn linecnt(&mut self) -> LINECNT_W<'_, LB1CFGRrs> {
        LINECNT_W::new(self, 16)
    }
}
/**CSI-2 Host line byte 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`lb1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lb1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:LB1CFGR)*/
pub struct LB1CFGRrs;
impl crate::RegisterSpec for LB1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`lb1cfgr::R`](R) reader structure
impl crate::Readable for LB1CFGRrs {}
///`write(|w| ..)` method takes [`lb1cfgr::W`](W) writer structure
impl crate::Writable for LB1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LB1CFGR to value 0
impl crate::Resettable for LB1CFGRrs {}
