///Register `EXTCFGR` reader
pub type R = crate::R<EXTCFGRrs>;
///Register `EXTCFGR` writer
pub type W = crate::W<EXTCFGRrs>;
/**HCLK3 shared prescaler (AHB3, Flash, and SRAM2)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SHDHPRE {
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
    ///0: SYSCLK not divided
    Div1 = 0,
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
///Field `SHDHPRE` reader - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
pub type SHDHPRE_R = crate::FieldReader<SHDHPRE>;
impl SHDHPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SHDHPRE {
        match self.bits {
            1 => SHDHPRE::Div3,
            2 => SHDHPRE::Div5,
            5 => SHDHPRE::Div6,
            6 => SHDHPRE::Div10,
            7 => SHDHPRE::Div32,
            8 => SHDHPRE::Div2,
            9 => SHDHPRE::Div4,
            10 => SHDHPRE::Div8,
            11 => SHDHPRE::Div16,
            12 => SHDHPRE::Div64,
            13 => SHDHPRE::Div128,
            14 => SHDHPRE::Div256,
            15 => SHDHPRE::Div512,
            _ => SHDHPRE::Div1,
        }
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
    ///SYSCLK not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), SHDHPRE::Div1)
    }
}
///Field `SHDHPRE` writer - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
pub type SHDHPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SHDHPRE, crate::Safe>;
impl<'a, REG> SHDHPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
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
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(SHDHPRE::Div1)
    }
}
/**HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHDHPREF {
    ///0: HCLK3 prescaler value not yet applied
    NotApplied = 0,
    ///1: HCLK3 prescaler value applied
    Applied = 1,
}
impl From<SHDHPREF> for bool {
    #[inline(always)]
    fn from(variant: SHDHPREF) -> Self {
        variant as u8 != 0
    }
}
///Field `SHDHPREF` reader - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
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
    ///HCLK3 prescaler value not yet applied
    #[inline(always)]
    pub fn is_not_applied(&self) -> bool {
        *self == SHDHPREF::NotApplied
    }
    ///HCLK3 prescaler value applied
    #[inline(always)]
    pub fn is_applied(&self) -> bool {
        *self == SHDHPREF::Applied
    }
}
impl R {
    ///Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpre(&self) -> SHDHPRE_R {
        SHDHPRE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 16 - HCLK3 shared prescaler flag (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpref(&self) -> SHDHPREF_R {
        SHDHPREF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTCFGR")
            .field("shdhpref", &self.shdhpref())
            .field("shdhpre", &self.shdhpre())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - HCLK3 shared prescaler (AHB3, Flash, and SRAM2)
    #[inline(always)]
    pub fn shdhpre(&mut self) -> SHDHPRE_W<'_, EXTCFGRrs> {
        SHDHPRE_W::new(self, 0)
    }
}
/**Extended clock recovery register

You can [`read`](crate::Reg::read) this register and get [`extcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:EXTCFGR)*/
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
