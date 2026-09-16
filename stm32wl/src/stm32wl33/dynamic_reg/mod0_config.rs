///Register `MOD0_CONFIG` reader
pub type R = crate::R<MOD0_CONFIGrs>;
///Register `MOD0_CONFIG` writer
pub type W = crate::W<MOD0_CONFIGrs>;
///Field `DATARATE_M` reader - The mantissa of the specified data rate (default: 38.
pub type DATARATE_M_R = crate::FieldReader<u16>;
///Field `DATARATE_M` writer - The mantissa of the specified data rate (default: 38.
pub type DATARATE_M_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `DATARATE_E` reader - The exponent of the specified data rate (default: 38.
pub type DATARATE_E_R = crate::FieldReader;
///Field `DATARATE_E` writer - The exponent of the specified data rate (default: 38.
pub type DATARATE_E_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MOD_TYPE` reader - Select the modulation type
pub type MOD_TYPE_R = crate::FieldReader;
///Field `MOD_TYPE` writer - Select the modulation type
pub type MOD_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CONST_MAP` reader - Also known as FOUR_GFSK_CONST_MAP
pub type CONST_MAP_R = crate::FieldReader;
///Field `CONST_MAP` writer - Also known as FOUR_GFSK_CONST_MAP
pub type CONST_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `BT_SEL` reader - Select BT value for GFSK
pub type BT_SEL_R = crate::BitReader;
///Field `BT_SEL` writer - Select BT value for GFSK
pub type BT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA_CLKON_LOCKONTX` reader - Enable the clock on analog PA in LOCKONTX state
pub type PA_CLKON_LOCKONTX_R = crate::BitReader;
///Field `PA_CLKON_LOCKONTX` writer - Enable the clock on analog PA in LOCKONTX state
pub type PA_CLKON_LOCKONTX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - The mantissa of the specified data rate (default: 38.
    #[inline(always)]
    pub fn datarate_m(&self) -> DATARATE_M_R {
        DATARATE_M_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:19 - The exponent of the specified data rate (default: 38.
    #[inline(always)]
    pub fn datarate_e(&self) -> DATARATE_E_R {
        DATARATE_E_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:22 - Select the modulation type
    #[inline(always)]
    pub fn mod_type(&self) -> MOD_TYPE_R {
        MOD_TYPE_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:25 - Also known as FOUR_GFSK_CONST_MAP
    #[inline(always)]
    pub fn const_map(&self) -> CONST_MAP_R {
        CONST_MAP_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - Select BT value for GFSK
    #[inline(always)]
    pub fn bt_sel(&self) -> BT_SEL_R {
        BT_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 31 - Enable the clock on analog PA in LOCKONTX state
    #[inline(always)]
    pub fn pa_clkon_lockontx(&self) -> PA_CLKON_LOCKONTX_R {
        PA_CLKON_LOCKONTX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOD0_CONFIG")
            .field("datarate_m", &self.datarate_m())
            .field("datarate_e", &self.datarate_e())
            .field("mod_type", &self.mod_type())
            .field("const_map", &self.const_map())
            .field("bt_sel", &self.bt_sel())
            .field("pa_clkon_lockontx", &self.pa_clkon_lockontx())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - The mantissa of the specified data rate (default: 38.
    #[inline(always)]
    pub fn datarate_m(&mut self) -> DATARATE_M_W<'_, MOD0_CONFIGrs> {
        DATARATE_M_W::new(self, 0)
    }
    ///Bits 16:19 - The exponent of the specified data rate (default: 38.
    #[inline(always)]
    pub fn datarate_e(&mut self) -> DATARATE_E_W<'_, MOD0_CONFIGrs> {
        DATARATE_E_W::new(self, 16)
    }
    ///Bits 20:22 - Select the modulation type
    #[inline(always)]
    pub fn mod_type(&mut self) -> MOD_TYPE_W<'_, MOD0_CONFIGrs> {
        MOD_TYPE_W::new(self, 20)
    }
    ///Bits 24:25 - Also known as FOUR_GFSK_CONST_MAP
    #[inline(always)]
    pub fn const_map(&mut self) -> CONST_MAP_W<'_, MOD0_CONFIGrs> {
        CONST_MAP_W::new(self, 24)
    }
    ///Bit 26 - Select BT value for GFSK
    #[inline(always)]
    pub fn bt_sel(&mut self) -> BT_SEL_W<'_, MOD0_CONFIGrs> {
        BT_SEL_W::new(self, 26)
    }
    ///Bit 31 - Enable the clock on analog PA in LOCKONTX state
    #[inline(always)]
    pub fn pa_clkon_lockontx(&mut self) -> PA_CLKON_LOCKONTX_W<'_, MOD0_CONFIGrs> {
        PA_CLKON_LOCKONTX_W::new(self, 31)
    }
}
/**MOD0_CONFIG register

You can [`read`](crate::Reg::read) this register and get [`mod0_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod0_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DYNAMIC_REG:MOD0_CONFIG)*/
pub struct MOD0_CONFIGrs;
impl crate::RegisterSpec for MOD0_CONFIGrs {
    type Ux = u32;
}
///`read()` method returns [`mod0_config::R`](R) reader structure
impl crate::Readable for MOD0_CONFIGrs {}
///`write(|w| ..)` method takes [`mod0_config::W`](W) writer structure
impl crate::Writable for MOD0_CONFIGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MOD0_CONFIG to value 0x0008_3a93
impl crate::Resettable for MOD0_CONFIGrs {
    const RESET_VALUE: u32 = 0x0008_3a93;
}
