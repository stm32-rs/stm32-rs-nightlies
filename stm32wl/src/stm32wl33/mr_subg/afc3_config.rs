///Register `AFC3_CONFIG` reader
pub type R = crate::R<AFC3_CONFIGrs>;
///Register `AFC3_CONFIG` writer
pub type W = crate::W<AFC3_CONFIGrs>;
///Field `AFC_INIT_MODE` reader - Control the initialization phase of the AFC and clock recovery algorithms:
pub type AFC_INIT_MODE_R = crate::BitReader;
///Field `AFC_INIT_MODE` writer - Control the initialization phase of the AFC and clock recovery algorithms:
pub type AFC_INIT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFC_SIGN_PERM_CHECK` reader - Enable the check of sign permanence of AFC corrected signal.
pub type AFC_SIGN_PERM_CHECK_R = crate::BitReader;
///Field `AFC_SIGN_PERM_CHECK` writer - Enable the check of sign permanence of AFC corrected signal.
pub type AFC_SIGN_PERM_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFC_TH_SIGN_PERM` reader - Threshold of chech sign permanence mechanism.
pub type AFC_TH_SIGN_PERM_R = crate::FieldReader;
///Field `AFC_TH_SIGN_PERM` writer - Threshold of chech sign permanence mechanism.
pub type AFC_TH_SIGN_PERM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AFC_REINIT_OPTION` reader - Select the AFC reinitialization option:
pub type AFC_REINIT_OPTION_R = crate::FieldReader;
///Field `AFC_REINIT_OPTION` writer - Select the AFC reinitialization option:
pub type AFC_REINIT_OPTION_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Control the initialization phase of the AFC and clock recovery algorithms:
    #[inline(always)]
    pub fn afc_init_mode(&self) -> AFC_INIT_MODE_R {
        AFC_INIT_MODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable the check of sign permanence of AFC corrected signal.
    #[inline(always)]
    pub fn afc_sign_perm_check(&self) -> AFC_SIGN_PERM_CHECK_R {
        AFC_SIGN_PERM_CHECK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Threshold of chech sign permanence mechanism.
    #[inline(always)]
    pub fn afc_th_sign_perm(&self) -> AFC_TH_SIGN_PERM_R {
        AFC_TH_SIGN_PERM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:7 - Select the AFC reinitialization option:
    #[inline(always)]
    pub fn afc_reinit_option(&self) -> AFC_REINIT_OPTION_R {
        AFC_REINIT_OPTION_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFC3_CONFIG")
            .field("afc_init_mode", &self.afc_init_mode())
            .field("afc_sign_perm_check", &self.afc_sign_perm_check())
            .field("afc_th_sign_perm", &self.afc_th_sign_perm())
            .field("afc_reinit_option", &self.afc_reinit_option())
            .finish()
    }
}
impl W {
    ///Bit 0 - Control the initialization phase of the AFC and clock recovery algorithms:
    #[inline(always)]
    pub fn afc_init_mode(&mut self) -> AFC_INIT_MODE_W<'_, AFC3_CONFIGrs> {
        AFC_INIT_MODE_W::new(self, 0)
    }
    ///Bit 1 - Enable the check of sign permanence of AFC corrected signal.
    #[inline(always)]
    pub fn afc_sign_perm_check(&mut self) -> AFC_SIGN_PERM_CHECK_W<'_, AFC3_CONFIGrs> {
        AFC_SIGN_PERM_CHECK_W::new(self, 1)
    }
    ///Bits 2:5 - Threshold of chech sign permanence mechanism.
    #[inline(always)]
    pub fn afc_th_sign_perm(&mut self) -> AFC_TH_SIGN_PERM_W<'_, AFC3_CONFIGrs> {
        AFC_TH_SIGN_PERM_W::new(self, 2)
    }
    ///Bits 6:7 - Select the AFC reinitialization option:
    #[inline(always)]
    pub fn afc_reinit_option(&mut self) -> AFC_REINIT_OPTION_W<'_, AFC3_CONFIGrs> {
        AFC_REINIT_OPTION_W::new(self, 6)
    }
}
/**AFC3_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`afc3_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afc3_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:AFC3_CONFIG)*/
pub struct AFC3_CONFIGrs;
impl crate::RegisterSpec for AFC3_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`afc3_config::R`](R) reader structure
impl crate::Readable for AFC3_CONFIGrs {}
///`write(|w| ..)` method takes [`afc3_config::W`](W) writer structure
impl crate::Writable for AFC3_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFC3_CONFIG to value 0xe8
impl crate::Resettable for AFC3_CONFIGrs {
    const RESET_VALUE: u32 = 0xe8;
}
