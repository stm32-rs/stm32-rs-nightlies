///Register `PATT3` reader
pub type R = crate::R<PATT3rs>;
///Register `PATT3` writer
pub type W = crate::W<PATT3rs>;
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
        f.debug_struct("PATT3")
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
    pub fn attsetx(&mut self) -> ATTSETX_W<'_, PATT3rs> {
        ATTSETX_W::new(self, 0)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<'_, PATT3rs> {
        ATTWAITX_W::new(self, 8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<'_, PATT3rs> {
        ATTHOLDX_W::new(self, 16)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W<'_, PATT3rs> {
        ATTHIZX_W::new(self, 24)
    }
}
/**Attribute memory space timing register 3

You can [`read`](crate::Reg::read) this register and get [`patt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#FMC:PATT3)*/
pub struct PATT3rs;
impl crate::RegisterSpec for PATT3rs {
    type Ux = u32;
}
///`read()` method returns [`patt3::R`](R) reader structure
impl crate::Readable for PATT3rs {}
///`write(|w| ..)` method takes [`patt3::W`](W) writer structure
impl crate::Writable for PATT3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PATT3 to value 0xfcfc_fcfc
impl crate::Resettable for PATT3rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
