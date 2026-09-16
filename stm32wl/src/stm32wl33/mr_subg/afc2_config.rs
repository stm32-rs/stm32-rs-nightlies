///Register `AFC2_CONFIG` reader
pub type R = crate::R<AFC2_CONFIGrs>;
///Register `AFC2_CONFIG` writer
pub type W = crate::W<AFC2_CONFIGrs>;
///Field `AFC_PD_LEAKAGE` reader - AFC Peak Detection leakage.
pub type AFC_PD_LEAKAGE_R = crate::FieldReader;
///Field `AFC_PD_LEAKAGE` writer - AFC Peak Detection leakage.
pub type AFC_PD_LEAKAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `AFC_MODE` reader - Select AFC mode:
pub type AFC_MODE_R = crate::BitReader;
///Field `AFC_MODE` writer - Select AFC mode:
pub type AFC_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFC_EN` reader - Enable AFC.
pub type AFC_EN_R = crate::BitReader;
///Field `AFC_EN` writer - Enable AFC.
pub type AFC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFC_FREEZE_ON_SYNC` reader - Freeze AFC correction upon SYNC word detection
pub type AFC_FREEZE_ON_SYNC_R = crate::BitReader;
///Field `AFC_FREEZE_ON_SYNC` writer - Freeze AFC correction upon SYNC word detection
pub type AFC_FREEZE_ON_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - AFC Peak Detection leakage.
    #[inline(always)]
    pub fn afc_pd_leakage(&self) -> AFC_PD_LEAKAGE_R {
        AFC_PD_LEAKAGE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - Select AFC mode:
    #[inline(always)]
    pub fn afc_mode(&self) -> AFC_MODE_R {
        AFC_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Enable AFC.
    #[inline(always)]
    pub fn afc_en(&self) -> AFC_EN_R {
        AFC_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Freeze AFC correction upon SYNC word detection
    #[inline(always)]
    pub fn afc_freeze_on_sync(&self) -> AFC_FREEZE_ON_SYNC_R {
        AFC_FREEZE_ON_SYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFC2_CONFIG")
            .field("afc_pd_leakage", &self.afc_pd_leakage())
            .field("afc_mode", &self.afc_mode())
            .field("afc_en", &self.afc_en())
            .field("afc_freeze_on_sync", &self.afc_freeze_on_sync())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - AFC Peak Detection leakage.
    #[inline(always)]
    pub fn afc_pd_leakage(&mut self) -> AFC_PD_LEAKAGE_W<'_, AFC2_CONFIGrs> {
        AFC_PD_LEAKAGE_W::new(self, 0)
    }
    ///Bit 5 - Select AFC mode:
    #[inline(always)]
    pub fn afc_mode(&mut self) -> AFC_MODE_W<'_, AFC2_CONFIGrs> {
        AFC_MODE_W::new(self, 5)
    }
    ///Bit 6 - Enable AFC.
    #[inline(always)]
    pub fn afc_en(&mut self) -> AFC_EN_W<'_, AFC2_CONFIGrs> {
        AFC_EN_W::new(self, 6)
    }
    ///Bit 7 - Freeze AFC correction upon SYNC word detection
    #[inline(always)]
    pub fn afc_freeze_on_sync(&mut self) -> AFC_FREEZE_ON_SYNC_W<'_, AFC2_CONFIGrs> {
        AFC_FREEZE_ON_SYNC_W::new(self, 7)
    }
}
/**AFC2_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc2_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc2_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC2_CONFIG)*/
pub struct AFC2_CONFIGrs;
impl crate::RegisterSpec for AFC2_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`afc2_config::R`](R) reader structure
impl crate::Readable for AFC2_CONFIGrs {}
///`write(|w| ..)` method takes [`afc2_config::W`](W) writer structure
impl crate::Writable for AFC2_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFC2_CONFIG to value 0xc8
impl crate::Resettable for AFC2_CONFIGrs {
    const RESET_VALUE: u32 = 0xc8;
}
