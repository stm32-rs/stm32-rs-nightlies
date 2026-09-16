///Register `CSICFGR` reader
pub type R = crate::R<CSICFGRrs>;
///Register `CSICFGR` writer
pub type W = crate::W<CSICFGRrs>;
///Field `CSITRIM` reader - CSITRIM
pub type CSITRIM_R = crate::FieldReader;
///Field `CSITRIM` writer - CSITRIM
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CSICAL` reader - CSICAL
pub type CSICAL_R = crate::FieldReader;
impl R {
    ///Bits 8:12 - CSITRIM
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:23 - CSICAL
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSICFGR")
            .field("csitrim", &self.csitrim())
            .field("csical", &self.csical())
            .finish()
    }
}
impl W {
    ///Bits 8:12 - CSITRIM
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W<'_, CSICFGRrs> {
        CSITRIM_W::new(self, 8)
    }
}
/**This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:CSICFGR)*/
pub struct CSICFGRrs;
impl crate::RegisterSpec for CSICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`csicfgr::R`](R) reader structure
impl crate::Readable for CSICFGRrs {}
///`write(|w| ..)` method takes [`csicfgr::W`](W) writer structure
impl crate::Writable for CSICFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSICFGR to value 0x1000
impl crate::Resettable for CSICFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
