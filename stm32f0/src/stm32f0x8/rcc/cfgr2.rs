///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
/**PREDIV division factor

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PREDIV {
    ///0: PREDIV input clock not divided
    Div1 = 0,
    ///1: PREDIV input clock divided by 2
    Div2 = 1,
    ///2: PREDIV input clock divided by 3
    Div3 = 2,
    ///3: PREDIV input clock divided by 4
    Div4 = 3,
    ///4: PREDIV input clock divided by 5
    Div5 = 4,
    ///5: PREDIV input clock divided by 6
    Div6 = 5,
    ///6: PREDIV input clock divided by 7
    Div7 = 6,
    ///7: PREDIV input clock divided by 8
    Div8 = 7,
    ///8: PREDIV input clock divided by 9
    Div9 = 8,
    ///9: PREDIV input clock divided by 10
    Div10 = 9,
    ///10: PREDIV input clock divided by 11
    Div11 = 10,
    ///11: PREDIV input clock divided by 12
    Div12 = 11,
    ///12: PREDIV input clock divided by 13
    Div13 = 12,
    ///13: PREDIV input clock divided by 14
    Div14 = 13,
    ///14: PREDIV input clock divided by 15
    Div15 = 14,
    ///15: PREDIV input clock divided by 16
    Div16 = 15,
}
impl From<PREDIV> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PREDIV {
    type Ux = u8;
}
impl crate::IsEnum for PREDIV {}
///Field `PREDIV` reader - PREDIV division factor
pub type PREDIV_R = crate::FieldReader<PREDIV>;
impl PREDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PREDIV {
        match self.bits {
            0 => PREDIV::Div1,
            1 => PREDIV::Div2,
            2 => PREDIV::Div3,
            3 => PREDIV::Div4,
            4 => PREDIV::Div5,
            5 => PREDIV::Div6,
            6 => PREDIV::Div7,
            7 => PREDIV::Div8,
            8 => PREDIV::Div9,
            9 => PREDIV::Div10,
            10 => PREDIV::Div11,
            11 => PREDIV::Div12,
            12 => PREDIV::Div13,
            13 => PREDIV::Div14,
            14 => PREDIV::Div15,
            15 => PREDIV::Div16,
            _ => unreachable!(),
        }
    }
    ///PREDIV input clock not divided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV::Div1
    }
    ///PREDIV input clock divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV::Div2
    }
    ///PREDIV input clock divided by 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV::Div3
    }
    ///PREDIV input clock divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV::Div4
    }
    ///PREDIV input clock divided by 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV::Div5
    }
    ///PREDIV input clock divided by 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV::Div6
    }
    ///PREDIV input clock divided by 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV::Div7
    }
    ///PREDIV input clock divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV::Div8
    }
    ///PREDIV input clock divided by 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV::Div9
    }
    ///PREDIV input clock divided by 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV::Div10
    }
    ///PREDIV input clock divided by 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV::Div11
    }
    ///PREDIV input clock divided by 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV::Div12
    }
    ///PREDIV input clock divided by 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV::Div13
    }
    ///PREDIV input clock divided by 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV::Div14
    }
    ///PREDIV input clock divided by 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV::Div15
    }
    ///PREDIV input clock divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV::Div16
    }
}
///Field `PREDIV` writer - PREDIV division factor
pub type PREDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PREDIV, crate::Safe>;
impl<'a, REG> PREDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PREDIV input clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div1)
    }
    ///PREDIV input clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div2)
    }
    ///PREDIV input clock divided by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div3)
    }
    ///PREDIV input clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div4)
    }
    ///PREDIV input clock divided by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div5)
    }
    ///PREDIV input clock divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div6)
    }
    ///PREDIV input clock divided by 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div7)
    }
    ///PREDIV input clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div8)
    }
    ///PREDIV input clock divided by 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div9)
    }
    ///PREDIV input clock divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div10)
    }
    ///PREDIV input clock divided by 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div11)
    }
    ///PREDIV input clock divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div12)
    }
    ///PREDIV input clock divided by 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div13)
    }
    ///PREDIV input clock divided by 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div14)
    }
    ///PREDIV input clock divided by 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div15)
    }
    ///PREDIV input clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PREDIV::Div16)
    }
}
impl R {
    ///Bits 0:3 - PREDIV division factor
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("prediv", &self.prediv())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - PREDIV division factor
    #[inline(always)]
    pub fn prediv(&mut self) -> PREDIV_W<'_, CFGR2rs> {
        PREDIV_W::new(self, 0)
    }
}
/**Clock configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x8.html#RCC:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
