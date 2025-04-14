///Register `PATT` reader
pub type R = crate::R<PATTrs>;
///Register `PATT` writer
pub type W = crate::W<PATTrs>;
///Field `ATTSETx` reader - ATTSETx
pub type ATTSETX_R = crate::FieldReader;
///Field `ATTSETx` writer - ATTSETx
pub type ATTSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTWAITx` reader - ATTWAITx
pub type ATTWAITX_R = crate::FieldReader;
///Field `ATTWAITx` writer - ATTWAITx
pub type ATTWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHOLDx` reader - ATTHOLDx
pub type ATTHOLDX_R = crate::FieldReader;
///Field `ATTHOLDx` writer - ATTHOLDx
pub type ATTHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHIZx` reader - ATTHIZx
pub type ATTHIZX_R = crate::FieldReader;
///Field `ATTHIZx` writer - ATTHIZx
pub type ATTHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PATT")
            .field("atthizx", &self.atthizx())
            .field("attholdx", &self.attholdx())
            .field("attwaitx", &self.attwaitx())
            .field("attsetx", &self.attsetx())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attsetx(&mut self) -> ATTSETX_W<PATTrs> {
        ATTSETX_W::new(self, 0)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<PATTrs> {
        ATTWAITX_W::new(self, 8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<PATTrs> {
        ATTHOLDX_W::new(self, 16)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W<PATTrs> {
        ATTHIZX_W::new(self, 24)
    }
}
/**Attribute memory space timing register 3

You can [`read`](crate::Reg::read) this register and get [`patt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#FMC:PATT)*/
pub struct PATTrs;
impl crate::RegisterSpec for PATTrs {
    type Ux = u32;
}
///`read()` method returns [`patt::R`](R) reader structure
impl crate::Readable for PATTrs {}
///`write(|w| ..)` method takes [`patt::W`](W) writer structure
impl crate::Writable for PATTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PATT to value 0xfcfc_fcfc
impl crate::Resettable for PATTrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
