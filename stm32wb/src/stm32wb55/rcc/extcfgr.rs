///Register `EXTCFGR` reader
pub type R = crate::R<EXTCFGRrs>;
///Register `EXTCFGR` writer
pub type W = crate::W<EXTCFGRrs>;
/**Shared AHB prescaler

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHDHPRE {
    ///0: SYSCLK not divided
    Div1 = 0,
    ///1: SYSCLK divided by 3
    Div3 = 1,
    ///2: SYSCLK divided by 5
    Div5 = 2,
    ///5: SYSCLK divided by 6
    Div6 = 5,
    ///6: SYSCLK divided by 10
    Div10 = 6,
    ///7: SYSCLK divided by 32
    Div32 = 7,
    ///8: SYSCLK divided by 2
    Div2 = 8,
    ///9: SYSCLK divided by 4
    Div4 = 9,
    ///10: SYSCLK divided by 8
    Div8 = 10,
    ///11: SYSCLK divided by 16
    Div16 = 11,
    ///12: SYSCLK divided by 64
    Div64 = 12,
    ///13: SYSCLK divided by 128
    Div128 = 13,
    ///14: SYSCLK divided by 256
    Div256 = 14,
    ///15: SYSCLK divided by 512
    Div512 = 15,
}
impl From<SHDHPRE> for u8 {
    #[inline(always)]
    fn from(variant: SHDHPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SHDHPRE {
    type Ux = u8;
}
impl crate::IsEnum for SHDHPRE {}
///Field `SHDHPRE` reader - Shared AHB prescaler
pub type SHDHPRE_R = crate::FieldReader<SHDHPRE>;
impl SHDHPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SHDHPRE> {
        match self.bits {
            0 => Some(SHDHPRE::Div1),
            1 => Some(SHDHPRE::Div3),
            2 => Some(SHDHPRE::Div5),
            5 => Some(SHDHPRE::Div6),
            6 => Some(SHDHPRE::Div10),
            7 => Some(SHDHPRE::Div32),
            8 => Some(SHDHPRE::Div2),
            9 => Some(SHDHPRE::Div4),
            10 => Some(SHDHPRE::Div8),
            11 => Some(SHDHPRE::Div16),
            12 => Some(SHDHPRE::Div64),
            13 => Some(SHDHPRE::Div128),
            14 => Some(SHDHPRE::Div256),
            15 => Some(SHDHPRE::Div512),
            _ => None,
        }
    }
    ///SYSCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SHDHPRE::Div1
    }
    ///SYSCLK divided by 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SHDHPRE::Div3
    }
    ///SYSCLK divided by 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == SHDHPRE::Div5
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == SHDHPRE::Div6
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == SHDHPRE::Div10
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SHDHPRE::Div32
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SHDHPRE::Div2
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SHDHPRE::Div4
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SHDHPRE::Div8
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SHDHPRE::Div16
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SHDHPRE::Div64
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SHDHPRE::Div128
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == SHDHPRE::Div256
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == SHDHPRE::Div512
    }
}
///Field `SHDHPRE` writer - Shared AHB prescaler
pub type SHDHPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SHDHPRE>;
impl<'a, REG> SHDHPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div1)
    }
    ///SYSCLK divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div3)
    }
    ///SYSCLK divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div5)
    }
    ///SYSCLK divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div6)
    }
    ///SYSCLK divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div10)
    }
    ///SYSCLK divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div32)
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div128)
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div512)
    }
}
///Field `C2HPRE` reader - CPU2 AHB prescaler
pub use SHDHPRE_R as C2HPRE_R;
///Field `C2HPRE` writer - CPU2 AHB prescaler
pub use SHDHPRE_W as C2HPRE_W;
/**Shared AHB prescaler flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHDHPREF {
    ///0: HCLK4 prescaler value not yet applied
    NotApplied = 0,
    ///1: HCLK4 prescaler value applied
    Applied = 1,
}
impl From<SHDHPREF> for bool {
    #[inline(always)]
    fn from(variant: SHDHPREF) -> Self {
        variant as u8 != 0
    }
}
///Field `SHDHPREF` reader - Shared AHB prescaler flag
pub type SHDHPREF_R = crate::BitReader<SHDHPREF>;
impl SHDHPREF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHDHPREF {
        match self.bits {
            false => SHDHPREF::NotApplied,
            true => SHDHPREF::Applied,
        }
    }
    ///HCLK4 prescaler value not yet applied
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == SHDHPREF::NotApplied
    }
    ///HCLK4 prescaler value applied
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == SHDHPREF::Applied
    }
}
/**CPU2 AHB prescaler flag

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2HPREF {
    ///0: HCLK2 prescaler value not yet applied
    NotApplied = 0,
    ///1: HCLK2 prescaler value applied
    Applied = 1,
}
impl From<C2HPREF> for bool {
    #[inline(always)]
    fn from(variant: C2HPREF) -> Self {
        variant as u8 != 0
    }
}
///Field `C2HPREF` reader - CPU2 AHB prescaler flag
pub type C2HPREF_R = crate::BitReader<C2HPREF>;
impl C2HPREF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C2HPREF {
        match self.bits {
            false => C2HPREF::NotApplied,
            true => C2HPREF::Applied,
        }
    }
    ///HCLK2 prescaler value not yet applied
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == C2HPREF::NotApplied
    }
    ///HCLK2 prescaler value applied
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == C2HPREF::Applied
    }
}
/**RF clock source selected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCSS {
    ///0: HSI16 used for radio system HCLK5 and APB3 clock
    Hsi16 = 0,
    ///1: HSE divided by 2 used for radio system HCLK5 and APB3 clock
    HseDiv2 = 1,
}
impl From<RFCSS> for bool {
    #[inline(always)]
    fn from(variant: RFCSS) -> Self {
        variant as u8 != 0
    }
}
///Field `RFCSS` reader - RF clock source selected
pub type RFCSS_R = crate::BitReader<RFCSS>;
impl RFCSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFCSS {
        match self.bits {
            false => RFCSS::Hsi16,
            true => RFCSS::HseDiv2,
        }
    }
    ///HSI16 used for radio system HCLK5 and APB3 clock
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == RFCSS::Hsi16
    }
    ///HSE divided by 2 used for radio system HCLK5 and APB3 clock
    #[inline(always)]
    pub fn is_hse_div2(&self) -> bool {
        *self == RFCSS::HseDiv2
    }
}
impl R {
    ///Bits 0:3 - Shared AHB prescaler
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CPU2 AHB prescaler
    #[inline(always)]
    pub fn c2hpre(&self) -> C2HPRE_R {
        C2HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 16 - Shared AHB prescaler flag
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU2 AHB prescaler flag
    #[inline(always)]
    pub fn c2hpref(&self) -> C2HPREF_R {
        C2HPREF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - RF clock source selected
    #[inline(always)]
    pub fn rfcss(&self) -> RFCSS_R {
        RFCSS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTCFGR")
            .field("rfcss", &self.rfcss())
            .field("c2hpref", &self.c2hpref())
            .field("shdhpref", &self.shdhpref())
            .field("shdhpre", &self.shdhpre())
            .field("c2hpre", &self.c2hpre())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Shared AHB prescaler
    #[inline(always)]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<'_, EXTCFGRrs> {
        SHDHPRE_W::new(self, 0)
    }
    ///Bits 4:7 - CPU2 AHB prescaler
    #[inline(always)]
    pub fn c2hpre(&mut self) -> C2HPRE_W<'_, EXTCFGRrs> {
        C2HPRE_W::new(self, 4)
    }
}
/**Extended clock recovery register

You can [`read`](crate::Reg::read) this register and get [`extcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:EXTCFGR)*/
pub struct EXTCFGRrs;
impl crate::RegisterSpec for EXTCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`extcfgr::R`](R) reader structure
impl crate::Readable for EXTCFGRrs {}
///`write(|w| ..)` method takes [`extcfgr::W`](W) writer structure
impl crate::Writable for EXTCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTCFGR to value 0x0003_0000
impl crate::Resettable for EXTCFGRrs {
    const RESET_VALUE: u32 = 0x0003_0000;
}
