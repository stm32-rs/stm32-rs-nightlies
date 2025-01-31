///Register `PATT2` reader
pub type R = crate::R<PATT2rs>;
///Register `PATT2` writer
pub type W = crate::W<PATT2rs>;
///Field `ATTSETx` reader - Attribute memory x setup time
pub type ATTSETX_R = crate::FieldReader;
///Field `ATTSETx` writer - Attribute memory x setup time
pub type ATTSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTWAITx` reader - Attribute memory x wait time
pub type ATTWAITX_R = crate::FieldReader;
///Field `ATTWAITx` writer - Attribute memory x wait time
pub type ATTWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHOLDx` reader - Attribute memory x hold time
pub type ATTHOLDX_R = crate::FieldReader;
///Field `ATTHOLDx` writer - Attribute memory x hold time
pub type ATTHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHIZx` reader - Attribute memory x databus HiZ time
pub type ATTHIZX_R = crate::FieldReader;
///Field `ATTHIZx` writer - Attribute memory x databus HiZ time
pub type ATTHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Attribute memory x setup time
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Attribute memory x wait time
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Attribute memory x hold time
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Attribute memory x databus HiZ time
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PATT2")
            .field("atthizx", &self.atthizx())
            .field("attholdx", &self.attholdx())
            .field("attwaitx", &self.attwaitx())
            .field("attsetx", &self.attsetx())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Attribute memory x setup time
    #[inline(always)]
    pub fn attsetx(&mut self) -> ATTSETX_W<PATT2rs> {
        ATTSETX_W::new(self, 0)
    }
    ///Bits 8:15 - Attribute memory x wait time
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<PATT2rs> {
        ATTWAITX_W::new(self, 8)
    }
    ///Bits 16:23 - Attribute memory x hold time
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<PATT2rs> {
        ATTHOLDX_W::new(self, 16)
    }
    ///Bits 24:31 - Attribute memory x databus HiZ time
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W<PATT2rs> {
        ATTHIZX_W::new(self, 24)
    }
}
/**Attribute memory space timing register 2

You can [`read`](crate::Reg::read) this register and get [`patt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#FSMC:PATT2)*/
pub struct PATT2rs;
impl crate::RegisterSpec for PATT2rs {
    type Ux = u32;
}
///`read()` method returns [`patt2::R`](R) reader structure
impl crate::Readable for PATT2rs {}
///`write(|w| ..)` method takes [`patt2::W`](W) writer structure
impl crate::Writable for PATT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PATT2 to value 0xfcfc_fcfc
impl crate::Resettable for PATT2rs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
