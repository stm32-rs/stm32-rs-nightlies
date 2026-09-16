///Register `FDCAN_CKDIV` reader
pub type R = crate::R<FDCAN_CKDIVrs>;
///Register `FDCAN_CKDIV` writer
pub type W = crate::W<FDCAN_CKDIVrs>;
/**input clock divider

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDIV {
    ///0: Divide by 1
    B0x0 = 0,
    ///1: Divide by 2
    B0x1 = 1,
    ///2: Divide by 4
    B0x2 = 2,
    ///3: Divide by 6
    B0x3 = 3,
    ///4: Divide by 8
    B0x4 = 4,
    ///5: Divide by 10
    B0x5 = 5,
    ///6: Divide by 12
    B0x6 = 6,
    ///7: Divide by 14
    B0x7 = 7,
    ///8: Divide by 16
    B0x8 = 8,
    ///9: Divide by 18
    B0x9 = 9,
    ///10: Divide by 20
    B0xA = 10,
    ///11: Divide by 22
    B0xB = 11,
    ///12: Divide by 24
    B0xC = 12,
    ///13: Divide by 26
    B0xD = 13,
    ///14: Divide by 28
    B0xE = 14,
    ///15: Divide by 30
    B0xF = 15,
}
impl From<PDIV> for u8 {
    #[inline(always)]
    fn from(variant: PDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PDIV {
    type Ux = u8;
}
impl crate::IsEnum for PDIV {}
///Field `PDIV` reader - input clock divider
pub type PDIV_R = crate::FieldReader<PDIV>;
impl PDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDIV {
        match self.bits {
            0 => PDIV::B0x0,
            1 => PDIV::B0x1,
            2 => PDIV::B0x2,
            3 => PDIV::B0x3,
            4 => PDIV::B0x4,
            5 => PDIV::B0x5,
            6 => PDIV::B0x6,
            7 => PDIV::B0x7,
            8 => PDIV::B0x8,
            9 => PDIV::B0x9,
            10 => PDIV::B0xA,
            11 => PDIV::B0xB,
            12 => PDIV::B0xC,
            13 => PDIV::B0xD,
            14 => PDIV::B0xE,
            15 => PDIV::B0xF,
            _ => unreachable!(),
        }
    }
    ///Divide by 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PDIV::B0x0
    }
    ///Divide by 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PDIV::B0x1
    }
    ///Divide by 4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PDIV::B0x2
    }
    ///Divide by 6
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PDIV::B0x3
    }
    ///Divide by 8
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PDIV::B0x4
    }
    ///Divide by 10
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PDIV::B0x5
    }
    ///Divide by 12
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PDIV::B0x6
    }
    ///Divide by 14
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PDIV::B0x7
    }
    ///Divide by 16
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PDIV::B0x8
    }
    ///Divide by 18
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PDIV::B0x9
    }
    ///Divide by 20
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PDIV::B0xA
    }
    ///Divide by 22
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PDIV::B0xB
    }
    ///Divide by 24
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == PDIV::B0xC
    }
    ///Divide by 26
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == PDIV::B0xD
    }
    ///Divide by 28
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == PDIV::B0xE
    }
    ///Divide by 30
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == PDIV::B0xF
    }
}
///Field `PDIV` writer - input clock divider
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PDIV, crate::Safe>;
impl<'a, REG> PDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide by 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x0)
    }
    ///Divide by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x1)
    }
    ///Divide by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x2)
    }
    ///Divide by 6
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x3)
    }
    ///Divide by 8
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x4)
    }
    ///Divide by 10
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x5)
    }
    ///Divide by 12
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x6)
    }
    ///Divide by 14
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x7)
    }
    ///Divide by 16
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x8)
    }
    ///Divide by 18
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0x9)
    }
    ///Divide by 20
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0xA)
    }
    ///Divide by 22
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0xB)
    }
    ///Divide by 24
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0xC)
    }
    ///Divide by 26
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0xD)
    }
    ///Divide by 28
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0xE)
    }
    ///Divide by 30
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV::B0xF)
    }
}
impl R {
    ///Bits 0:3 - input clock divider
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CKDIV")
            .field("pdiv", &self.pdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - input clock divider
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W<'_, FDCAN_CKDIVrs> {
        PDIV_W::new(self, 0)
    }
}
/**FDCAN CFG clock divider register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_CKDIV)*/
pub struct FDCAN_CKDIVrs;
impl crate::RegisterSpec for FDCAN_CKDIVrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ckdiv::R`](R) reader structure
impl crate::Readable for FDCAN_CKDIVrs {}
///`write(|w| ..)` method takes [`fdcan_ckdiv::W`](W) writer structure
impl crate::Writable for FDCAN_CKDIVrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_CKDIV to value 0
impl crate::Resettable for FDCAN_CKDIVrs {}
