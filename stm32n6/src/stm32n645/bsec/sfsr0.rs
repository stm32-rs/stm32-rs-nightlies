///Register `SFSR0` reader
pub type R = crate::R<SFSR0rs>;
///Field `SFW0` reader - Shadowed fuse word 0
pub type SFW0_R = crate::BitReader;
///Field `SFW1` reader - Shadowed fuse word 1
pub type SFW1_R = crate::BitReader;
///Field `SFW2` reader - Shadowed fuse word 2
pub type SFW2_R = crate::BitReader;
///Field `SFW3` reader - Shadowed fuse word 3
pub type SFW3_R = crate::BitReader;
///Field `SFW4` reader - Shadowed fuse word 4
pub type SFW4_R = crate::BitReader;
///Field `SFW5` reader - Shadowed fuse word 5
pub type SFW5_R = crate::BitReader;
///Field `SFW6` reader - Shadowed fuse word 6
pub type SFW6_R = crate::BitReader;
///Field `SFW7` reader - Shadowed fuse word 7
pub type SFW7_R = crate::BitReader;
///Field `SFW8` reader - Shadowed fuse word 8
pub type SFW8_R = crate::BitReader;
///Field `SFW9` reader - Shadowed fuse word 9
pub type SFW9_R = crate::BitReader;
///Field `SFW10` reader - Shadowed fuse word 10
pub type SFW10_R = crate::BitReader;
///Field `SFW11` reader - Shadowed fuse word 11
pub type SFW11_R = crate::BitReader;
///Field `SFW12` reader - Shadowed fuse word 12
pub type SFW12_R = crate::BitReader;
///Field `SFW13` reader - Shadowed fuse word 13
pub type SFW13_R = crate::BitReader;
///Field `SFW14` reader - Shadowed fuse word 14
pub type SFW14_R = crate::BitReader;
///Field `SFW15` reader - Shadowed fuse word 15
pub type SFW15_R = crate::BitReader;
///Field `SFW16` reader - Shadowed fuse word 16
pub type SFW16_R = crate::BitReader;
///Field `SFW17` reader - Shadowed fuse word 17
pub type SFW17_R = crate::BitReader;
///Field `SFW18` reader - Shadowed fuse word 18
pub type SFW18_R = crate::BitReader;
///Field `SFW19` reader - Shadowed fuse word 19
pub type SFW19_R = crate::BitReader;
///Field `SFW20` reader - Shadowed fuse word 20
pub type SFW20_R = crate::BitReader;
///Field `SFW21` reader - Shadowed fuse word 21
pub type SFW21_R = crate::BitReader;
///Field `SFW22` reader - Shadowed fuse word 22
pub type SFW22_R = crate::BitReader;
///Field `SFW23` reader - Shadowed fuse word 23
pub type SFW23_R = crate::BitReader;
///Field `SFW24` reader - Shadowed fuse word 24
pub type SFW24_R = crate::BitReader;
///Field `SFW25` reader - Shadowed fuse word 25
pub type SFW25_R = crate::BitReader;
///Field `SFW26` reader - Shadowed fuse word 26
pub type SFW26_R = crate::BitReader;
///Field `SFW27` reader - Shadowed fuse word 27
pub type SFW27_R = crate::BitReader;
///Field `SFW28` reader - Shadowed fuse word 28
pub type SFW28_R = crate::BitReader;
///Field `SFW29` reader - Shadowed fuse word 29
pub type SFW29_R = crate::BitReader;
///Field `SFW30` reader - Shadowed fuse word 30
pub type SFW30_R = crate::BitReader;
///Field `SFW31` reader - Shadowed fuse word 31
pub type SFW31_R = crate::BitReader;
impl R {
    ///Bit 0 - Shadowed fuse word 0
    #[inline(always)]
    pub fn sfw0(&self) -> SFW0_R {
        SFW0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Shadowed fuse word 1
    #[inline(always)]
    pub fn sfw1(&self) -> SFW1_R {
        SFW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Shadowed fuse word 2
    #[inline(always)]
    pub fn sfw2(&self) -> SFW2_R {
        SFW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Shadowed fuse word 3
    #[inline(always)]
    pub fn sfw3(&self) -> SFW3_R {
        SFW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Shadowed fuse word 4
    #[inline(always)]
    pub fn sfw4(&self) -> SFW4_R {
        SFW4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Shadowed fuse word 5
    #[inline(always)]
    pub fn sfw5(&self) -> SFW5_R {
        SFW5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Shadowed fuse word 6
    #[inline(always)]
    pub fn sfw6(&self) -> SFW6_R {
        SFW6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Shadowed fuse word 7
    #[inline(always)]
    pub fn sfw7(&self) -> SFW7_R {
        SFW7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Shadowed fuse word 8
    #[inline(always)]
    pub fn sfw8(&self) -> SFW8_R {
        SFW8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Shadowed fuse word 9
    #[inline(always)]
    pub fn sfw9(&self) -> SFW9_R {
        SFW9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Shadowed fuse word 10
    #[inline(always)]
    pub fn sfw10(&self) -> SFW10_R {
        SFW10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Shadowed fuse word 11
    #[inline(always)]
    pub fn sfw11(&self) -> SFW11_R {
        SFW11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Shadowed fuse word 12
    #[inline(always)]
    pub fn sfw12(&self) -> SFW12_R {
        SFW12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Shadowed fuse word 13
    #[inline(always)]
    pub fn sfw13(&self) -> SFW13_R {
        SFW13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Shadowed fuse word 14
    #[inline(always)]
    pub fn sfw14(&self) -> SFW14_R {
        SFW14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Shadowed fuse word 15
    #[inline(always)]
    pub fn sfw15(&self) -> SFW15_R {
        SFW15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Shadowed fuse word 16
    #[inline(always)]
    pub fn sfw16(&self) -> SFW16_R {
        SFW16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Shadowed fuse word 17
    #[inline(always)]
    pub fn sfw17(&self) -> SFW17_R {
        SFW17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Shadowed fuse word 18
    #[inline(always)]
    pub fn sfw18(&self) -> SFW18_R {
        SFW18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Shadowed fuse word 19
    #[inline(always)]
    pub fn sfw19(&self) -> SFW19_R {
        SFW19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Shadowed fuse word 20
    #[inline(always)]
    pub fn sfw20(&self) -> SFW20_R {
        SFW20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Shadowed fuse word 21
    #[inline(always)]
    pub fn sfw21(&self) -> SFW21_R {
        SFW21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Shadowed fuse word 22
    #[inline(always)]
    pub fn sfw22(&self) -> SFW22_R {
        SFW22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Shadowed fuse word 23
    #[inline(always)]
    pub fn sfw23(&self) -> SFW23_R {
        SFW23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Shadowed fuse word 24
    #[inline(always)]
    pub fn sfw24(&self) -> SFW24_R {
        SFW24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Shadowed fuse word 25
    #[inline(always)]
    pub fn sfw25(&self) -> SFW25_R {
        SFW25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Shadowed fuse word 26
    #[inline(always)]
    pub fn sfw26(&self) -> SFW26_R {
        SFW26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Shadowed fuse word 27
    #[inline(always)]
    pub fn sfw27(&self) -> SFW27_R {
        SFW27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Shadowed fuse word 28
    #[inline(always)]
    pub fn sfw28(&self) -> SFW28_R {
        SFW28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Shadowed fuse word 29
    #[inline(always)]
    pub fn sfw29(&self) -> SFW29_R {
        SFW29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Shadowed fuse word 30
    #[inline(always)]
    pub fn sfw30(&self) -> SFW30_R {
        SFW30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Shadowed fuse word 31
    #[inline(always)]
    pub fn sfw31(&self) -> SFW31_R {
        SFW31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFSR0")
            .field("sfw0", &self.sfw0())
            .field("sfw1", &self.sfw1())
            .field("sfw2", &self.sfw2())
            .field("sfw3", &self.sfw3())
            .field("sfw4", &self.sfw4())
            .field("sfw5", &self.sfw5())
            .field("sfw6", &self.sfw6())
            .field("sfw7", &self.sfw7())
            .field("sfw8", &self.sfw8())
            .field("sfw9", &self.sfw9())
            .field("sfw10", &self.sfw10())
            .field("sfw11", &self.sfw11())
            .field("sfw12", &self.sfw12())
            .field("sfw13", &self.sfw13())
            .field("sfw14", &self.sfw14())
            .field("sfw15", &self.sfw15())
            .field("sfw16", &self.sfw16())
            .field("sfw17", &self.sfw17())
            .field("sfw18", &self.sfw18())
            .field("sfw19", &self.sfw19())
            .field("sfw20", &self.sfw20())
            .field("sfw21", &self.sfw21())
            .field("sfw22", &self.sfw22())
            .field("sfw23", &self.sfw23())
            .field("sfw24", &self.sfw24())
            .field("sfw25", &self.sfw25())
            .field("sfw26", &self.sfw26())
            .field("sfw27", &self.sfw27())
            .field("sfw28", &self.sfw28())
            .field("sfw29", &self.sfw29())
            .field("sfw30", &self.sfw30())
            .field("sfw31", &self.sfw31())
            .finish()
    }
}
/**BSEC shadowed fuses status register 0

You can [`read`](crate::Reg::read) this register and get [`sfsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:SFSR0)*/
pub struct SFSR0rs;
impl crate::RegisterSpec for SFSR0rs {
    type Ux = u32;
}
///`read()` method returns [`sfsr0::R`](R) reader structure
impl crate::Readable for SFSR0rs {}
///`reset()` method sets SFSR0 to value 0
impl crate::Resettable for SFSR0rs {}
