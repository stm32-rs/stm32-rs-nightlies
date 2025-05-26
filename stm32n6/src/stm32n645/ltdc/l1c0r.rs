///Register `L1C0R` reader
pub type R = crate::R<L1C0Rrs>;
///Field `CKTA` reader - color key transparency ability
pub type CKTA_R = crate::BitReader;
///Field `CFBDA` reader - color frame buffer duplication ability
pub type CFBDA_R = crate::BitReader;
///Field `CFBPA` reader - color frame buffer pitch ability
pub type CFBPA_R = crate::BitReader;
///Field `APA` reader - alpha plane ability
pub type APA_R = crate::BitReader;
///Field `DCP` reader - default color programmability
pub type DCP_R = crate::BitReader;
///Field `WINA` reader - windowing ability
pub type WINA_R = crate::BitReader;
///Field `CLUTA` reader - CLUT ability
pub type CLUTA_R = crate::BitReader;
///Field `CKRA` reader - color key replace ability
pub type CKRA_R = crate::BitReader;
///Field `F21` reader - blending factor 2, ability for: 1.0
pub type F21_R = crate::BitReader;
///Field `F20` reader - blending factor 2, ability for: 0.0
pub type F20_R = crate::BitReader;
///Field `F2P` reader - blending factor 2, ability for: pixel_alpha
pub type F2P_R = crate::BitReader;
///Field `F21P` reader - blending factor 2, ability for: 1.0 - pixel_alpha
pub type F21P_R = crate::BitReader;
///Field `F2C` reader - blending factor 2, ability for: constant_alpha
pub type F2C_R = crate::BitReader;
///Field `F21C` reader - blending factor 2, ability for: 1.0 - constant_alpha
pub type F21C_R = crate::BitReader;
///Field `F2PC` reader - blending factor 2, ability for: pixel_alpha * constant_alpha
pub type F2PC_R = crate::BitReader;
///Field `F21PC` reader - blending factor 2, ability for: 1.0 - (pixel_alpha * constant_alpha)
pub type F21PC_R = crate::BitReader;
///Field `F11` reader - blending factor 1, ability for: 1.0
pub type F11_R = crate::BitReader;
///Field `F10` reader - blending factor 1,ability for: 0.0
pub type F10_R = crate::BitReader;
///Field `F1P` reader - blending factor 1, ability for: pixel_alpha
pub type F1P_R = crate::BitReader;
///Field `F11P` reader - blending factor 1, ability for: 1.0 - pixel_alpha
pub type F11P_R = crate::BitReader;
///Field `F1C` reader - blending factor 1, ability for: constant_alpha
pub type F1C_R = crate::BitReader;
///Field `F11C` reader - blending factor 1, ability for: 1.0 - constant_alpha
pub type F11C_R = crate::BitReader;
///Field `F1PC` reader - blending factor 1, ability for: pixel_alpha * constant_alpha
pub type F1PC_R = crate::BitReader;
///Field `F11PC` reader - blending factor 1, ability for: 1.0 - (pixel_alpha * constant_alpha)
pub type F11PC_R = crate::BitReader;
///Field `FF` reader - flexible pixel format, ability
pub type FF_R = crate::BitReader;
///Field `RGB888` reader - pixel format, ability for rgb888
pub type RGB888_R = crate::BitReader;
///Field `BGR565` reader - pixel format, ability for bgr565
pub type BGR565_R = crate::BitReader;
///Field `RGB565` reader - pixel format, ability for rgb565
pub type RGB565_R = crate::BitReader;
///Field `BGRA888` reader - pixel format, ability for bgra8888
pub type BGRA888_R = crate::BitReader;
///Field `RGBA8888` reader - pixel format, ability for rgba8888
pub type RGBA8888_R = crate::BitReader;
///Field `ABGR8888` reader - pixel format, ability for abgr8888
pub type ABGR8888_R = crate::BitReader;
///Field `ARGB8888` reader - pixel format, ability for argb8888
pub type ARGB8888_R = crate::BitReader;
impl R {
    ///Bit 0 - color key transparency ability
    #[inline(always)]
    pub fn ckta(&self) -> CKTA_R {
        CKTA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - color frame buffer duplication ability
    #[inline(always)]
    pub fn cfbda(&self) -> CFBDA_R {
        CFBDA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - color frame buffer pitch ability
    #[inline(always)]
    pub fn cfbpa(&self) -> CFBPA_R {
        CFBPA_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - alpha plane ability
    #[inline(always)]
    pub fn apa(&self) -> APA_R {
        APA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - default color programmability
    #[inline(always)]
    pub fn dcp(&self) -> DCP_R {
        DCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - windowing ability
    #[inline(always)]
    pub fn wina(&self) -> WINA_R {
        WINA_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CLUT ability
    #[inline(always)]
    pub fn cluta(&self) -> CLUTA_R {
        CLUTA_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - color key replace ability
    #[inline(always)]
    pub fn ckra(&self) -> CKRA_R {
        CKRA_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - blending factor 2, ability for: 1.0
    #[inline(always)]
    pub fn f21(&self) -> F21_R {
        F21_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - blending factor 2, ability for: 0.0
    #[inline(always)]
    pub fn f20(&self) -> F20_R {
        F20_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - blending factor 2, ability for: pixel_alpha
    #[inline(always)]
    pub fn f2p(&self) -> F2P_R {
        F2P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - blending factor 2, ability for: 1.0 - pixel_alpha
    #[inline(always)]
    pub fn f21p(&self) -> F21P_R {
        F21P_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - blending factor 2, ability for: constant_alpha
    #[inline(always)]
    pub fn f2c(&self) -> F2C_R {
        F2C_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - blending factor 2, ability for: 1.0 - constant_alpha
    #[inline(always)]
    pub fn f21c(&self) -> F21C_R {
        F21C_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - blending factor 2, ability for: pixel_alpha * constant_alpha
    #[inline(always)]
    pub fn f2pc(&self) -> F2PC_R {
        F2PC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - blending factor 2, ability for: 1.0 - (pixel_alpha * constant_alpha)
    #[inline(always)]
    pub fn f21pc(&self) -> F21PC_R {
        F21PC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - blending factor 1, ability for: 1.0
    #[inline(always)]
    pub fn f11(&self) -> F11_R {
        F11_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - blending factor 1,ability for: 0.0
    #[inline(always)]
    pub fn f10(&self) -> F10_R {
        F10_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - blending factor 1, ability for: pixel_alpha
    #[inline(always)]
    pub fn f1p(&self) -> F1P_R {
        F1P_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - blending factor 1, ability for: 1.0 - pixel_alpha
    #[inline(always)]
    pub fn f11p(&self) -> F11P_R {
        F11P_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - blending factor 1, ability for: constant_alpha
    #[inline(always)]
    pub fn f1c(&self) -> F1C_R {
        F1C_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - blending factor 1, ability for: 1.0 - constant_alpha
    #[inline(always)]
    pub fn f11c(&self) -> F11C_R {
        F11C_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - blending factor 1, ability for: pixel_alpha * constant_alpha
    #[inline(always)]
    pub fn f1pc(&self) -> F1PC_R {
        F1PC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - blending factor 1, ability for: 1.0 - (pixel_alpha * constant_alpha)
    #[inline(always)]
    pub fn f11pc(&self) -> F11PC_R {
        F11PC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - flexible pixel format, ability
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - pixel format, ability for rgb888
    #[inline(always)]
    pub fn rgb888(&self) -> RGB888_R {
        RGB888_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - pixel format, ability for bgr565
    #[inline(always)]
    pub fn bgr565(&self) -> BGR565_R {
        BGR565_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - pixel format, ability for rgb565
    #[inline(always)]
    pub fn rgb565(&self) -> RGB565_R {
        RGB565_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - pixel format, ability for bgra8888
    #[inline(always)]
    pub fn bgra888(&self) -> BGRA888_R {
        BGRA888_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - pixel format, ability for rgba8888
    #[inline(always)]
    pub fn rgba8888(&self) -> RGBA8888_R {
        RGBA8888_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - pixel format, ability for abgr8888
    #[inline(always)]
    pub fn abgr8888(&self) -> ABGR8888_R {
        ABGR8888_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - pixel format, ability for argb8888
    #[inline(always)]
    pub fn argb8888(&self) -> ARGB8888_R {
        ARGB8888_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1C0R")
            .field("ckta", &self.ckta())
            .field("cfbda", &self.cfbda())
            .field("cfbpa", &self.cfbpa())
            .field("apa", &self.apa())
            .field("dcp", &self.dcp())
            .field("wina", &self.wina())
            .field("cluta", &self.cluta())
            .field("ckra", &self.ckra())
            .field("f21", &self.f21())
            .field("f20", &self.f20())
            .field("f2p", &self.f2p())
            .field("f21p", &self.f21p())
            .field("f2c", &self.f2c())
            .field("f21c", &self.f21c())
            .field("f2pc", &self.f2pc())
            .field("f21pc", &self.f21pc())
            .field("f11", &self.f11())
            .field("f10", &self.f10())
            .field("f1p", &self.f1p())
            .field("f11p", &self.f11p())
            .field("f1c", &self.f1c())
            .field("f11c", &self.f11c())
            .field("f1pc", &self.f1pc())
            .field("f11pc", &self.f11pc())
            .field("ff", &self.ff())
            .field("rgb888", &self.rgb888())
            .field("bgr565", &self.bgr565())
            .field("rgb565", &self.rgb565())
            .field("bgra888", &self.bgra888())
            .field("rgba8888", &self.rgba8888())
            .field("abgr8888", &self.abgr8888())
            .field("argb8888", &self.argb8888())
            .finish()
    }
}
/**LTDC layerx configuration 0 register

You can [`read`](crate::Reg::read) this register and get [`l1c0r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1C0R)*/
pub struct L1C0Rrs;
impl crate::RegisterSpec for L1C0Rrs {
    type Ux = u32;
}
///`read()` method returns [`l1c0r::R`](R) reader structure
impl crate::Readable for L1C0Rrs {}
///`reset()` method sets L1C0R to value 0xff50_a076
impl crate::Resettable for L1C0Rrs {
    const RESET_VALUE: u32 = 0xff50_a076;
}
