///Register `PATT` reader
pub type R = crate::R<PATTrs>;
///Register `PATT` writer
pub type W = crate::W<PATTrs>;
///Field `ATTSET` reader - ATTSETx
pub type ATTSET_R = crate::FieldReader;
///Field `ATTSET` writer - ATTSETx
pub type ATTSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTWAIT` reader - ATTWAITx
pub type ATTWAIT_R = crate::FieldReader;
///Field `ATTWAIT` writer - ATTWAITx
pub type ATTWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHOLD` reader - ATTHOLDx
pub type ATTHOLD_R = crate::FieldReader;
///Field `ATTHOLD` writer - ATTHOLDx
pub type ATTHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ATTHIZ` reader - ATTHIZx
pub type ATTHIZ_R = crate::FieldReader;
///Field `ATTHIZ` writer - ATTHIZx
pub type ATTHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PATT")
            .field("atthiz", &self.atthiz())
            .field("atthold", &self.atthold())
            .field("attwait", &self.attwait())
            .field("attset", &self.attset())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ATTSETx
    #[inline(always)]
    pub fn attset(&mut self) -> ATTSET_W<'_, PATTrs> {
        ATTSET_W::new(self, 0)
    }
    ///Bits 8:15 - ATTWAITx
    #[inline(always)]
    pub fn attwait(&mut self) -> ATTWAIT_W<'_, PATTrs> {
        ATTWAIT_W::new(self, 8)
    }
    ///Bits 16:23 - ATTHOLDx
    #[inline(always)]
    pub fn atthold(&mut self) -> ATTHOLD_W<'_, PATTrs> {
        ATTHOLD_W::new(self, 16)
    }
    ///Bits 24:31 - ATTHIZx
    #[inline(always)]
    pub fn atthiz(&mut self) -> ATTHIZ_W<'_, PATTrs> {
        ATTHIZ_W::new(self, 24)
    }
}
/**Attribute memory space timing register

You can [`read`](crate::Reg::read) this register and get [`patt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#FMC:PATT)*/
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
