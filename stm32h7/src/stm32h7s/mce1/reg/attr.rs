///Register `ATTR` reader
pub type R = crate::R<ATTRrs>;
///Register `ATTR` writer
pub type W = crate::W<ATTRrs>;
///Field `WREN` reader - Write enable This bit is taken into account only if BREN is set.
pub type WREN_R = crate::BitReader;
///Field `WREN` writer - Write enable This bit is taken into account only if BREN is set.
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Write enable This bit is taken into account only if BREN is set.
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATTR").field("wren", &self.wren()).finish()
    }
}
impl W {
    ///Bit 16 - Write enable This bit is taken into account only if BREN is set.
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<'_, ATTRrs> {
        WREN_W::new(self, 16)
    }
}
/**Region attribute register

You can [`read`](crate::Reg::read) this register and get [`attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ATTRrs;
impl crate::RegisterSpec for ATTRrs {
    type Ux = u32;
}
///`read()` method returns [`attr::R`](R) reader structure
impl crate::Readable for ATTRrs {}
///`write(|w| ..)` method takes [`attr::W`](W) writer structure
impl crate::Writable for ATTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ATTR to value 0
impl crate::Resettable for ATTRrs {}
