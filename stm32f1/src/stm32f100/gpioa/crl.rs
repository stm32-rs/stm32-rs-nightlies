///Register `CRL` reader
pub type R = crate::R<CRLrs>;
///Register `CRL` writer
pub type W = crate::W<CRLrs>;
/**Port n.%s mode bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    ///0: Input mode (reset state)
    Input = 0,
    ///1: Output mode 10 MHz
    Output = 1,
    ///2: Output mode 2 MHz
    Output2 = 2,
    ///3: Output mode 50 MHz
    Output50 = 3,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
///Field `MODE(0-7)` reader - Port n.%s mode bits
pub type MODE_R = crate::FieldReader<MODE>;
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MODE {
        match self.bits {
            0 => MODE::Input,
            1 => MODE::Output,
            2 => MODE::Output2,
            3 => MODE::Output50,
            _ => unreachable!(),
        }
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE::Input
    }
    ///Output mode 10 MHz
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == MODE::Output
    }
    ///Output mode 2 MHz
    #[inline(always)]
    pub fn is_output2(&self) -> bool {
        *self == MODE::Output2
    }
    ///Output mode 50 MHz
    #[inline(always)]
    pub fn is_output50(&self) -> bool {
        *self == MODE::Output50
    }
}
///Field `MODE(0-7)` writer - Port n.%s mode bits
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE, crate::Safe>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Input)
    }
    ///Output mode 10 MHz
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Output)
    }
    ///Output mode 2 MHz
    #[inline(always)]
    pub fn output2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Output2)
    }
    ///Output mode 50 MHz
    #[inline(always)]
    pub fn output50(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Output50)
    }
}
/**Port n.%s configuration bits

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CONFIG {
    ///0: Analog mode / Push-Pull mode
    PushPull = 0,
    ///1: Floating input (reset state) / Open Drain-Mode
    OpenDrain = 1,
    ///2: Input with pull-up/pull-down / Alternate Function Push-Pull Mode
    AltPushPull = 2,
    ///3: Alternate Function Open-Drain Mode
    AltOpenDrain = 3,
}
impl From<CONFIG> for u8 {
    #[inline(always)]
    fn from(variant: CONFIG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CONFIG {
    type Ux = u8;
}
impl crate::IsEnum for CONFIG {}
///Field `CNF(0-7)` reader - Port n.%s configuration bits
pub type CNF_R = crate::FieldReader<CONFIG>;
impl CNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CONFIG {
        match self.bits {
            0 => CONFIG::PushPull,
            1 => CONFIG::OpenDrain,
            2 => CONFIG::AltPushPull,
            3 => CONFIG::AltOpenDrain,
            _ => unreachable!(),
        }
    }
    ///Analog mode / Push-Pull mode
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == CONFIG::PushPull
    }
    ///Floating input (reset state) / Open Drain-Mode
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == CONFIG::OpenDrain
    }
    ///Input with pull-up/pull-down / Alternate Function Push-Pull Mode
    #[inline(always)]
    pub fn is_alt_push_pull(&self) -> bool {
        *self == CONFIG::AltPushPull
    }
    ///Alternate Function Open-Drain Mode
    #[inline(always)]
    pub fn is_alt_open_drain(&self) -> bool {
        *self == CONFIG::AltOpenDrain
    }
}
///Field `CNF(0-7)` writer - Port n.%s configuration bits
pub type CNF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CONFIG, crate::Safe>;
impl<'a, REG> CNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Analog mode / Push-Pull mode
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIG::PushPull)
    }
    ///Floating input (reset state) / Open Drain-Mode
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIG::OpenDrain)
    }
    ///Input with pull-up/pull-down / Alternate Function Push-Pull Mode
    #[inline(always)]
    pub fn alt_push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIG::AltPushPull)
    }
    ///Alternate Function Open-Drain Mode
    #[inline(always)]
    pub fn alt_open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(CONFIG::AltOpenDrain)
    }
}
impl R {
    ///Port n.(0-7) mode bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE0` field.</div>
    #[inline(always)]
    pub fn mode(&self, n: u8) -> MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_R::new(((self.bits >> (n * 4)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port n.(0-7) mode bits
    #[inline(always)]
    pub fn mode_iter(&self) -> impl Iterator<Item = MODE_R> + '_ {
        (0..8).map(move |n| MODE_R::new(((self.bits >> (n * 4)) & 3) as u8))
    }
    ///Bits 0:1 - Port n.0 mode bits
    #[inline(always)]
    pub fn mode0(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Port n.1 mode bits
    #[inline(always)]
    pub fn mode1(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Port n.2 mode bits
    #[inline(always)]
    pub fn mode2(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Port n.3 mode bits
    #[inline(always)]
    pub fn mode3(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - Port n.4 mode bits
    #[inline(always)]
    pub fn mode4(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - Port n.5 mode bits
    #[inline(always)]
    pub fn mode5(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - Port n.6 mode bits
    #[inline(always)]
    pub fn mode6(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - Port n.7 mode bits
    #[inline(always)]
    pub fn mode7(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Port n.(0-7) configuration bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CNF0` field.</div>
    #[inline(always)]
    pub fn cnf(&self, n: u8) -> CNF_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port n.(0-7) configuration bits
    #[inline(always)]
    pub fn cnf_iter(&self) -> impl Iterator<Item = CNF_R> + '_ {
        (0..8).map(move |n| CNF_R::new(((self.bits >> (n * 4 + 2)) & 3) as u8))
    }
    ///Bits 2:3 - Port n.0 configuration bits
    #[inline(always)]
    pub fn cnf0(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 6:7 - Port n.1 configuration bits
    #[inline(always)]
    pub fn cnf1(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 10:11 - Port n.2 configuration bits
    #[inline(always)]
    pub fn cnf2(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 14:15 - Port n.3 configuration bits
    #[inline(always)]
    pub fn cnf3(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 18:19 - Port n.4 configuration bits
    #[inline(always)]
    pub fn cnf4(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 22:23 - Port n.5 configuration bits
    #[inline(always)]
    pub fn cnf5(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 26:27 - Port n.6 configuration bits
    #[inline(always)]
    pub fn cnf6(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 30:31 - Port n.7 configuration bits
    #[inline(always)]
    pub fn cnf7(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRL")
            .field("mode0", &self.mode0())
            .field("mode1", &self.mode1())
            .field("mode2", &self.mode2())
            .field("mode3", &self.mode3())
            .field("mode4", &self.mode4())
            .field("mode5", &self.mode5())
            .field("mode6", &self.mode6())
            .field("mode7", &self.mode7())
            .field("cnf0", &self.cnf0())
            .field("cnf1", &self.cnf1())
            .field("cnf2", &self.cnf2())
            .field("cnf3", &self.cnf3())
            .field("cnf4", &self.cnf4())
            .field("cnf5", &self.cnf5())
            .field("cnf6", &self.cnf6())
            .field("cnf7", &self.cnf7())
            .finish()
    }
}
impl W {
    ///Port n.(0-7) mode bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `MODE0` field.</div>
    #[inline(always)]
    pub fn mode(&mut self, n: u8) -> MODE_W<'_, CRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        MODE_W::new(self, n * 4)
    }
    ///Bits 0:1 - Port n.0 mode bits
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 0)
    }
    ///Bits 4:5 - Port n.1 mode bits
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 4)
    }
    ///Bits 8:9 - Port n.2 mode bits
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 8)
    }
    ///Bits 12:13 - Port n.3 mode bits
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 12)
    }
    ///Bits 16:17 - Port n.4 mode bits
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 16)
    }
    ///Bits 20:21 - Port n.5 mode bits
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 20)
    }
    ///Bits 24:25 - Port n.6 mode bits
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 24)
    }
    ///Bits 28:29 - Port n.7 mode bits
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE_W<'_, CRLrs> {
        MODE_W::new(self, 28)
    }
    ///Port n.(0-7) configuration bits
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CNF0` field.</div>
    #[inline(always)]
    pub fn cnf(&mut self, n: u8) -> CNF_W<'_, CRLrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNF_W::new(self, n * 4 + 2)
    }
    ///Bits 2:3 - Port n.0 configuration bits
    #[inline(always)]
    pub fn cnf0(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 2)
    }
    ///Bits 6:7 - Port n.1 configuration bits
    #[inline(always)]
    pub fn cnf1(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 6)
    }
    ///Bits 10:11 - Port n.2 configuration bits
    #[inline(always)]
    pub fn cnf2(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 10)
    }
    ///Bits 14:15 - Port n.3 configuration bits
    #[inline(always)]
    pub fn cnf3(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 14)
    }
    ///Bits 18:19 - Port n.4 configuration bits
    #[inline(always)]
    pub fn cnf4(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 18)
    }
    ///Bits 22:23 - Port n.5 configuration bits
    #[inline(always)]
    pub fn cnf5(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 22)
    }
    ///Bits 26:27 - Port n.6 configuration bits
    #[inline(always)]
    pub fn cnf6(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 26)
    }
    ///Bits 30:31 - Port n.7 configuration bits
    #[inline(always)]
    pub fn cnf7(&mut self) -> CNF_W<'_, CRLrs> {
        CNF_W::new(self, 30)
    }
}
/**Port configuration register low (GPIOn_CRL)

You can [`read`](crate::Reg::read) this register and get [`crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#GPIOA:CRL)*/
pub struct CRLrs;
impl crate::RegisterSpec for CRLrs {
    type Ux = u32;
}
///`read()` method returns [`crl::R`](R) reader structure
impl crate::Readable for CRLrs {}
///`write(|w| ..)` method takes [`crl::W`](W) writer structure
impl crate::Writable for CRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRL to value 0x4444_4444
impl crate::Resettable for CRLrs {
    const RESET_VALUE: u32 = 0x4444_4444;
}
